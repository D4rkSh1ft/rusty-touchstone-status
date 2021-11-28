use scraper::ElementRef;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RFParameters {
    pub downstream_parameters: Vec<DownstreamParameter>,
    pub upstream_parameters: Vec<UpstreamParameter>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpstreamParameter {
    pub id: u8,
    pub channel_id: u8,
    pub freq_mhz: f32,
    pub power_dbmv: f32,
    pub modulation: String,
    pub channel_type: String,
    pub symbol_rate: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DownstreamParameter {
    pub id: u8,
    pub channel_id: u8,
    pub freq_mhz: f32,
    pub power_dbmv: f32,
    pub modulation: String,
    pub snr_db: f32,
    pub octets: usize,
    pub corrected_count: u8,
    pub uncorrectable_count: u8,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StatusParameter {
    pub uptime: String,
    pub computers_detected: String,
    pub cm_status: String,
    pub current_datetime: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InterfaceInformation {
    pub name: String,
    pub provisioned: String,
    pub state: String,
    pub speed: String,
    pub mac_address: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ArrisStatus {
    pub rf_parameters: RFParameters,
    pub status: StatusParameter,
    pub interfaces: Vec<InterfaceInformation>,
}

pub fn take_first_string(input: Option<ElementRef>) -> String {
    input.unwrap().text().collect::<String>()
}

pub fn take_first_string_from_split(input: Option<ElementRef>) -> String {
    take_first_string(input)
        .split_whitespace()
        .next()
        .unwrap()
        .to_string()
}
pub fn take_last_string_from_split(input: Option<ElementRef>) -> String {
    take_first_string(input)
        .split_whitespace()
        .last()
        .unwrap()
        .to_string()
}


pub fn parse_tables_rf_parameters_upstream_table(
    tables_rf_parameters_upstream: scraper::ElementRef,
) -> Vec<UpstreamParameter> {
    let tr_selector = scraper::Selector::parse("tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let mut table_iter = tables_rf_parameters_upstream.select(&tr_selector);

    // Skip first row.
    table_iter.next();

    let mut td_tags = table_iter.next().unwrap().select(&td_selector);

    let upstream_parameter = UpstreamParameter {
        id: take_last_string_from_split(td_tags.next())
            .parse::<u8>()
            .unwrap(),

        channel_id: take_first_string(td_tags.next())
            .parse::<u8>()
            .unwrap_or(0),

        freq_mhz: take_first_string_from_split(td_tags.next())
            .parse::<f32>()
            .unwrap(),

        power_dbmv: take_first_string_from_split(td_tags.next())
            .parse::<f32>()
            .unwrap_or(0.0),

        channel_type: take_first_string(td_tags.next()),

        symbol_rate: take_first_string(td_tags.next()),

        modulation: take_first_string(td_tags.next()),
    };

    return vec![upstream_parameter];
}

pub fn parse_tables_rf_parameters_downstream_table(
    tables_rf_parameters_downstream: scraper::ElementRef,
) -> Vec<DownstreamParameter> {
    let tr_selector = scraper::Selector::parse("tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let mut table_iter = tables_rf_parameters_downstream.select(&tr_selector);

    // Skip first row.
    table_iter.next();

    table_iter
        .map(|datarow| {
            let mut td_tags = datarow.select(&td_selector);

            DownstreamParameter {
                id: take_last_string_from_split(td_tags.next())
                    .parse()
                    .unwrap(),

                channel_id: take_first_string(td_tags.next())
                    .parse::<u8>()
                    .unwrap_or(0),

                freq_mhz: take_first_string_from_split(td_tags.next())
                    .parse::<f32>()
                    .unwrap(),

                power_dbmv: take_first_string_from_split(td_tags.next())
                    .parse::<f32>()
                    .unwrap_or(0.0),

                snr_db: take_first_string_from_split(td_tags.next())
                    .parse::<f32>()
                    .unwrap_or(0.0),

                modulation: take_first_string(td_tags.next()),

                octets: take_first_string(td_tags.next())
                    .parse::<usize>()
                    .unwrap_or(0),

                corrected_count: take_first_string(td_tags.next())
                    .parse::<u8>()
                    .unwrap_or(0),

                uncorrectable_count: take_first_string(td_tags.next())
                    .parse::<u8>()
                    .unwrap_or(0),
            }
        })
        .collect::<Vec<DownstreamParameter>>()
}

pub fn parse_tables_interface_parameters_table(
    tables_interface_parameters: scraper::ElementRef,
) -> Vec<InterfaceInformation> {
    let tr_selector = scraper::Selector::parse("tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let mut table_iter = tables_interface_parameters.select(&tr_selector);

    // Skip first row.
    table_iter.next();

    table_iter
        .map(|datarow| {
            let mut td_tags = datarow.select(&td_selector);

            InterfaceInformation {
                name: take_first_string(td_tags.next()),

                provisioned: take_first_string(td_tags.next()),

                state: take_first_string(td_tags.next()),

                speed: take_first_string(td_tags.next()),

                mac_address: take_first_string(td_tags.next()),
            }
        })
        .collect::<Vec<InterfaceInformation>>()
}

pub fn parse_tables_status_table(tables_status: scraper::ElementRef) -> StatusParameter {
    let tr_selector = scraper::Selector::parse("tr").unwrap();
    let td_selector = scraper::Selector::parse("td").unwrap();

    let mut table_iter = tables_status.select(&tr_selector);

    StatusParameter {
        uptime: take_first_string(table_iter.next().unwrap().select(&td_selector).last()),

        computers_detected: take_first_string(
            table_iter.next().unwrap().select(&td_selector).last(),
        )
        .trim()
        .to_owned(),

        cm_status: take_first_string(
            table_iter.next().unwrap().select(&td_selector).last(),
        ),

        current_datetime: take_first_string(
            table_iter.next().unwrap().select(&td_selector).last(),
        ),
    }
}

pub fn parse_request(html: &str) -> ArrisStatus {
    let document = scraper::Html::parse_document(html);

    let table_selector = scraper::Selector::parse("table").unwrap();

    let mut tables = document.select(&table_selector);

    // Skip row
    let _tables_rf_parameters = tables.next().unwrap();

    let tables_rf_parameters_downstream = tables.next().unwrap();
    let downstream_parameters =
        parse_tables_rf_parameters_downstream_table(tables_rf_parameters_downstream);

    // Skip row
    let _tables_rf_parameters_fec_counters = tables.next().unwrap();

    let tables_rf_parameters_upstream = tables.next().unwrap();
    let upstream_parameters =
        parse_tables_rf_parameters_upstream_table(tables_rf_parameters_upstream);

    // Skip row
    let _tables_status = tables.next().unwrap();

    let tables_status = tables.next().unwrap();
    let tables_status_values = parse_tables_status_table(tables_status);

    // Skip row
    let _tables_interface_parameters = tables.next().unwrap();

    let tables_interface_parameters_values = tables.next().unwrap();
    let interface_parameters =
        parse_tables_interface_parameters_table(tables_interface_parameters_values);

    let rf_parameters = RFParameters {
        downstream_parameters,
        upstream_parameters,
    };

    ArrisStatus {
        rf_parameters,
        status: tables_status_values,
        interfaces: interface_parameters,
    }
}
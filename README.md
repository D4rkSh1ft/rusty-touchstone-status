# Summary
This simple tool grabs the cable status information from an Arris
cable modem. The output is semi-json.

By default this status page is served via HTTP at the cable modem's
ip address.

Example: `http://192.168.100.1/`

# Reference Hardware
This was tested with a the following hardware:
|      |                                                    |
|------|----------------------------------------------------|
|System|ARRIS DOCSIS 3.0 / PC 1.5 Touchstone Telephony Modem|
|HW_REV|3                                                   |
|VENDOR|ARRIS Group, Inc.                                   |
|BOOTR|2.2.0.45                                             |
|SW_REV|9.1.103J6J                                          |
|MODEL|TM1602A                                              | 

# Example Output
```
{
  "rf_parameters": {
    "downstream_parameters": [
      {
        "id": 1,
        "channel_id": 5,
        "freq_mhz": 465.0,
        "power_dbmv": -5.3,
        "modulation": "256QAM",
        "snr_db": 40.37,
        "octets": 253449450,
        "corrected_count": 5,
        "uncorrectable_count": 0
      },
      {
        "id": 2,
        "channel_id": 1,
        "freq_mhz": 435.0,
        "power_dbmv": -6.3,
        "modulation": "256QAM",
        "snr_db": 38.61,
        "octets": 197846562,
        "corrected_count": 14,
        "uncorrectable_count": 0
      },
      {
        "id": 3,
        "channel_id": 2,
        "freq_mhz": 441.0,
        "power_dbmv": -6.0,
        "modulation": "256QAM",
        "snr_db": 38.98,
        "octets": 206476437,
        "corrected_count": 11,
        "uncorrectable_count": 0
      },
      {
        "id": 4,
        "channel_id": 3,
        "freq_mhz": 453.0,
        "power_dbmv": -5.9,
        "modulation": "256QAM",
        "snr_db": 38.98,
        "octets": 209010634,
        "corrected_count": 18,
        "uncorrectable_count": 0
      },
      {
        "id": 5,
        "channel_id": 4,
        "freq_mhz": 459.0,
        "power_dbmv": -5.8,
        "modulation": "256QAM",
        "snr_db": 40.37,
        "octets": 208644858,
        "corrected_count": 18,
        "uncorrectable_count": 0
      },
      {
        "id": 6,
        "channel_id": 6,
        "freq_mhz": 471.0,
        "power_dbmv": -5.1,
        "modulation": "256QAM",
        "snr_db": 38.98,
        "octets": 217095595,
        "corrected_count": 7,
        "uncorrectable_count": 0
      },
      {
        "id": 7,
        "channel_id": 7,
        "freq_mhz": 477.0,
        "power_dbmv": 0.0,
        "modulation": "----",
        "snr_db": 0.0,
        "octets": 0,
        "corrected_count": 0,
        "uncorrectable_count": 0
      },
      {
        "id": 8,
        "channel_id": 10,
        "freq_mhz": 495.0,
        "power_dbmv": -4.7,
        "modulation": "256QAM",
        "snr_db": 40.37,
        "octets": 239671327,
        "corrected_count": 2,
        "uncorrectable_count": 0
      },
      {
        "id": 9,
        "channel_id": 11,
        "freq_mhz": 507.0,
        "power_dbmv": -4.6,
        "modulation": "256QAM",
        "snr_db": 38.98,
        "octets": 232174258,
        "corrected_count": 4,
        "uncorrectable_count": 0
      },
      {
        "id": 10,
        "channel_id": 14,
        "freq_mhz": 525.0,
        "power_dbmv": -4.6,
        "modulation": "256QAM",
        "snr_db": 38.61,
        "octets": 243169878,
        "corrected_count": 4,
        "uncorrectable_count": 0
      },
      {
        "id": 11,
        "channel_id": 19,
        "freq_mhz": 573.0,
        "power_dbmv": -4.4,
        "modulation": "256QAM",
        "snr_db": 40.37,
        "octets": 257087452,
        "corrected_count": 17,
        "uncorrectable_count": 0
      },
      {
        "id": 12,
        "channel_id": 20,
        "freq_mhz": 579.0,
        "power_dbmv": -4.4,
        "modulation": "256QAM",
        "snr_db": 40.37,
        "octets": 213336229,
        "corrected_count": 16,
        "uncorrectable_count": 0
      },
      {
        "id": 13,
        "channel_id": 21,
        "freq_mhz": 585.0,
        "power_dbmv": -4.2,
        "modulation": "256QAM",
        "snr_db": 30.59,
        "octets": 192788931,
        "corrected_count": 0,
        "uncorrectable_count": 0
      },
      {
        "id": 14,
        "channel_id": 22,
        "freq_mhz": 591.0,
        "power_dbmv": -3.9,
        "modulation": "256QAM",
        "snr_db": 34.48,
        "octets": 219949989,
        "corrected_count": 0,
        "uncorrectable_count": 0
      },
      {
        "id": 15,
        "channel_id": 23,
        "freq_mhz": 597.0,
        "power_dbmv": -4.0,
        "modulation": "256QAM",
        "snr_db": 38.98,
        "octets": 239570826,
        "corrected_count": 10,
        "uncorrectable_count": 0
      },
      {
        "id": 16,
        "channel_id": 24,
        "freq_mhz": 603.0,
        "power_dbmv": -3.7,
        "modulation": "256QAM",
        "snr_db": 40.37,
        "octets": 244383382,
        "corrected_count": 5,
        "uncorrectable_count": 0
      },
      {
        "id": 17,
        "channel_id": 26,
        "freq_mhz": 609.0,
        "power_dbmv": -3.4,
        "modulation": "256QAM",
        "snr_db": 40.95,
        "octets": 237169491,
        "corrected_count": 1,
        "uncorrectable_count": 0
      },
      {
        "id": 18,
        "channel_id": 27,
        "freq_mhz": 615.0,
        "power_dbmv": -3.4,
        "modulation": "256QAM",
        "snr_db": 40.95,
        "octets": 245231649,
        "corrected_count": 2,
        "uncorrectable_count": 0
      },
      {
        "id": 19,
        "channel_id": 28,
        "freq_mhz": 621.0,
        "power_dbmv": -3.5,
        "modulation": "256QAM",
        "snr_db": 38.98,
        "octets": 269378053,
        "corrected_count": 1,
        "uncorrectable_count": 0
      },
      {
        "id": 20,
        "channel_id": 29,
        "freq_mhz": 627.0,
        "power_dbmv": -3.5,
        "modulation": "256QAM",
        "snr_db": 38.98,
        "octets": 274578540,
        "corrected_count": 7,
        "uncorrectable_count": 0
      },
      {
        "id": 21,
        "channel_id": 30,
        "freq_mhz": 633.0,
        "power_dbmv": -3.2,
        "modulation": "256QAM",
        "snr_db": 38.98,
        "octets": 287245462,
        "corrected_count": 28,
        "uncorrectable_count": 0
      },
      {
        "id": 22,
        "channel_id": 31,
        "freq_mhz": 639.0,
        "power_dbmv": -2.9,
        "modulation": "256QAM",
        "snr_db": 40.37,
        "octets": 282887406,
        "corrected_count": 5,
        "uncorrectable_count": 0
      },
      {
        "id": 23,
        "channel_id": 32,
        "freq_mhz": 645.0,
        "power_dbmv": -2.4,
        "modulation": "256QAM",
        "snr_db": 40.37,
        "octets": 283036231,
        "corrected_count": 0,
        "uncorrectable_count": 0
      },
      {
        "id": 24,
        "channel_id": 33,
        "freq_mhz": 651.0,
        "power_dbmv": -2.3,
        "modulation": "256QAM",
        "snr_db": 40.37,
        "octets": 286115609,
        "corrected_count": 2,
        "uncorrectable_count": 0
      }
    ],
    "upstream_parameters": [
      {
        "id": 1,
        "channel_id": 68,
        "freq_mhz": 36.8,
        "power_dbmv": 57.0,
        "modulation": "64QAM",
        "channel_type": "DOCSIS2.0 (ATDMA)",
        "symbol_rate": "5120 kSym/s"
      }
    ]
  },
  "status": {
    "uptime": "0 d: 16 h: 19  m",
    "computers_detected": "staticCPE(1), dynamicCPE(1)",
    "cm_status": "OPERATIONAL",
    "current_datetime": "Sun 2021-11-28 18:33:06"
  },
  "interfaces": [
    {
      "name": "LAN",
      "provisioned": "Enabled",
      "state": "Up",
      "speed": "1000(Full)",
      "mac_address": "00:00:00:00:00:00"
    },
    {
      "name": "CABLE",
      "provisioned": "Enabled",
      "state": "Up",
      "speed": "-----",
      "mac_address": "00:00:00:00:00:00"
    },
    {
      "name": "MTA",
      "provisioned": "NotInitiated",
      "state": "Down",
      "speed": "-----",
      "mac_address": "00:00:00:00:00:00"
    }
  ]
}
```

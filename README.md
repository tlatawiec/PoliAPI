# PoliTracker is a Politician Trade Tracker Designed to Allow you to Take Advantage of Inside Trades

## Overview


The Politician Trade Tracker, also known as PoliTracker, is a comprehensive system designed to provide transparent access to the trading activities of government officials. PoliTracker consists of two main components:

1. **PoliAPI:** A robust API that allows users to query detailed information about trades made by government officials. The API offers endpoints to filter trades by issuer, trade type, politician name, and other criteria, making it easy to analyze and monitor the trading activities of public officials.

2. **PoliBot:** An upcoming Twitter bot that will automatically post updates about new trades made by politicians. PoliBot aims to keep the public informed in real-time, enhancing transparency and accountability.

**Usage**

  ENDPOINTS:
    
    /api/politician/{politician_name} | (trades by certain politician)
    
    /api/publish_date/recent | (last two weeks of published trades)
    
    /api/published_within/{x} | (last x weeks of published trades)
    
    /api/traded_date/recent | (last two weeks of trades)
    
    /api/traded_within/{x} | (last x weeks of trades)
    
    /api/price/over/{x} | (trades with a price over x)
    
    /api/price/under/{x} | (trades with a price under x)
    
    /api/price/na | (trades with N/A price)
    
    /api/size/{x} | (0-8) (trade size) (0 -> 1K-15K | 1 -> 15K-50K | 2 -> 50K-100K | 3 -> 100K-250K | 4 -> 250K-500K | 5 -> 500K-1M | 6 -> 1M-5M | 7 -> 5M-25M | 8 -> 25M-50M)
    
    /api/issuer/{issuer_name} | (get all trades by issuer)
    
    /api/type/{type} | (buy / sell)

  Example Request:
  
     api.poliapi.com/politician/Nancy Pelosi

  Example Response:
  
    {
      "Politician Name:": "Nancy Pelosi",
      "Politician Party:": "Democrat",
      "Politician Position:": "House",
      "Politician State:": "CA",
      "Price:": "N/A",
      "Publish Date:": "2022-05-04",
      "Reporting Gap:": "22",
      "Size:": "50K–100K",
      "Trade Issuer:": "AT&T Inc",
      "Traded Date:": "2022-04-11",
      "Type:": "exchange"
    }

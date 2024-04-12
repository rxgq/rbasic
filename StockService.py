import yfinance as yf

class StockService:
    def __init__(self):
        pass

    @staticmethod
    def fetch_stock_info(stock):
        stock = yf.Ticker(stock)
        print(stock.info)

    def MFST(self):
        return self.fetch_stock_info("MSFT")
    
    def AAPL(self):
        return self.fetch_stock_info("AAPL")
    
    def GOOG(self):
        return self.fetch_stock_info("GOOG")
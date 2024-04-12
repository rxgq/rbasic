import yfinance as yf

Stocks = {
    "MICROSOFT": "MSFT",
    "GOOGLE": "GOOG",
    "APPLE": "AAPL",
}

class Stock:
    def __init__(self, stock):
        self.stock = stock

    def fetch_stock_history(self):
        info = yf.Ticker(self.stock)
        history = info.history(period="1mo")

        return history
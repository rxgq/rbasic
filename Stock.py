import yfinance as yf

symbols = {
    "MICROSOFT": "MSFT",
    "GOOGLE": "GOOG",
    "APPLE": "AAPL",
}

class Stock:
    def __init__(self, stock):
        self.stock = stock

    def fetch_history(self, period):
        info = yf.Ticker(self.stock)
        history = info.history(period)

        return history
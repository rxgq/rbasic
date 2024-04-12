from StockService import Stock, Stocks

import yfinance as yf

def main():
    stock = Stock(Stocks["MICROSOFT"])
    stock.fetch_stock_history()

    
if __name__ == "__main__":
    main()
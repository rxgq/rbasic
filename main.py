from Stock import Stock, symbols
import matplotlib.pyplot as plt
import mplfinance as mpf

def main():
    stock = Stock(symbols["MICROSOFT"])
    history = stock.fetch_history("1mo")

    plot_candlestick_chart(history)
    



def plot_candlestick_chart(data):
    mpf.plot(data, type='candle', style='charles', volume=True)


if __name__ == "__main__":
    main()
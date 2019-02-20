import perf

def one_million():
    "{:,}".format(1_000_000)

def main():
    runner = perf.Runner()
    runner.timeit("one_million",
                  "one_million()",
                  "from __main__ import one_million",
                  inner_loops=10)

if __name__ == "__main__":
    main()


def sliding_window(data, window_size):
    for i in range(len(data) - window_size + 1):
        yield data[i:i + window_size]

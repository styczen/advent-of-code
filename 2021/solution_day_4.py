import numpy as np


if __name__ == '__main__':
    with open('input_day_4', 'r') as f:
        data = f.read()
    
    data = data.split('\n\n')
    chosen_numbers = [int(x) for x in data.pop(0).split(',')]
    boards = []
    for raw_board_str in data:
        board_str = raw_board_str.replace('\n\n', ' ').split()
        board = np.array([int(x) for x in board_str])
        board = board.reshape(5, 5)
        boards.append(board)
    boards = np.array(boards)
    hit_map = np.zeros(boards.shape, dtype=np.uint8)
    
    won_boards_idx = []
    last_result_val = None
    result_val = None
    for number in chosen_numbers:
        for i, board in enumerate(boards):
            try:
                # If that call does not raise exception
                # that means that this board already won
                # so skip it
                won_boards_idx.index(i)
                continue
            except ValueError:
                pass
            # Mark the number on the hit map
            hit_map[i][board == number] = 1
            # Check whether this board wins
            sum_col = np.sum(hit_map[i], axis=0)
            sum_row = np.sum(hit_map[i], axis=1)
            if np.max(sum_col) == 5 or np.max(sum_row) == 5:
                # Save index of board which just won
                won_boards_idx.append(i)
                if result_val is None:
                    result_val = np.sum(board[hit_map[i] == 0]) * number
                last_result_val = np.sum(board[hit_map[i] == 0]) * number
    print('result:', result_val)
    print('last board:', last_result_val)


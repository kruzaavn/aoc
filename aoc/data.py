import requests
import pathlib

session_id = pathlib.Path().cwd().joinpath('session.txt').read_text().strip()


class BadConnectionError(Exception):
    pass


def get_data(year, day):

    response = requests.get(f'https://www.adventofcode.com/{year}/day/{day}/input', cookies={'session': session_id})

    if response.status_code != 200:
        raise BadConnectionError(response.content.decode('utf-8'))

    return response.content.decode('utf-8')


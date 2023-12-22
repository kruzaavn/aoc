import requests
import pathlib

session_id = pathlib.Path().cwd().joinpath('session.txt').read_text().strip()


def get_data(year, day):

    response = requests.get(f'https://www.adventofcode.com/{year}/day/{day}/input', cookies={'session': session_id})

    return response.content.decode('utf-8')


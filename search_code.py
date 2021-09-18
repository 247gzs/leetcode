import os

base_path = os.path.abspath(os.path.dirname(__file__))


def get_search_path(suffix_path):
    return os.path.join(base_path, suffix_path)


def get_file_list(file_path):
    return [os.path.join(file_path, filename) for filename in os.listdir(file_path) if filename.endswith('.rs')]


def search_file_info(filename, search_info):
    with open(filename, 'r') as fr:
        for line in fr.readlines():
            if search_info in line:
                print(filename)
                return


def enter_search(search_info):
    file_list = get_file_list(get_search_path("rust"))
    for file in file_list:
        search_file_info(file, search_info)


if __name__ == "__main__":
    enter_search("map")

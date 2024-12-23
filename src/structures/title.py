import re
from src.utils.utils import get_title_from_prefix


def find_title_data(title_id: str, data: str) -> str:
    title_data = re.findall(r'\n%s={.+?\n}' % title_id, data, re.S)
    return title_data[0]


def inject_title(raw_data: str, find_key: str, base_name: str) -> str:
    if 'article' in raw_data:
        find_art = re.findall(r'article="(.*?)"', raw_data, re.S)
        return find_art[0] + base_name

    return get_title_from_prefix(find_key, base_name)


def get_title_name(title_id: str, data: str) -> str:
    """
    Extracts and returns the name of a title given its ID and the raw data.

    Args:
        title_id (str): The ID of the title to search for.
        data (str): The raw data containing title information.

    Returns:
        dict: A dictionary containing the title ID and the injected title.

    Raises:
        IndexError: If the title data or name cannot be found in the raw data.

    Example:
        >>> data = '...raw data...'
        >>> get_title_name('125', data)
        {'id': '125', 'title': 'Kingdom of France'}
    """
    if title_id is None:
        return {}
    
    # Find the raw data for the given title ID
    raw_data = find_title_data(title_id, data)
    
    # Extract the key and name from the raw data
    find_key = re.findall(r'key=(.*?)\n', raw_data, re.S)[0]
    find_name = re.findall(r'name="(.*?)"', raw_data, re.S)
    
    # Get the base name of the title
    base_name = find_name[0]

    title_dict = {
        "id": title_id,
        "title": inject_title(raw_data, find_key, base_name)
    }
    
    # Inject the title and return the result
    return title_dict

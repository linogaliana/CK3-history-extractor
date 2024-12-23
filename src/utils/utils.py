from collections import defaultdict
from datetime import datetime

def combine_dicts(dicts):
    """
    Combine a list of dictionaries into a single dictionary. For each key, 
    the values from all dictionaries are collected into a list. If a key 
    is missing in a dictionary, None is used as a placeholder. If a key 
    has only one value across all dictionaries, the value is not wrapped 
    in a list.
    Args:
        dicts (list of dict): A list of dictionaries to combine.
    Returns:
        dict: A combined dictionary with lists of values or single values.
    Example:
        >>> dicts = [{'a': 1, 'b': 2}, {'a': 3, 'c': 4}]
        >>> combine_dicts(dicts)
        {'a': [1, 3], 'b': [2, None] 'c': [None, 4]}
    """
    combined = defaultdict(list)
    all_keys = set()

    # Collect all keys
    for d in dicts:
        all_keys.update(d.keys())

    # Combine dictionaries, filling missing keys with None
    for d in dicts:
        for key in all_keys:
            combined[key].append(d.get(key, None))
    
    # Flatten lists with a single item
    for key, value in combined.items():
        if len(value) == 1:
            combined[key] = value[0]
    
    return dict(combined)


def convert_ck3_date(date_str: str):
        # Split the date string into year, month, and day
        parts = date_str.split('.')
        if len(parts) != 3:
            return None
        
        year, month, day = parts
        
        # Pad year, month, and day with leading zeros if necessary
        year = year.zfill(4)
        month = month.zfill(2)
        day = day.zfill(2)
        
        # Construct the new date string
        new_date_str = f"{year}-{month}-{day}"

        # Convert to datetime
        return datetime.strptime(new_date_str, '%Y-%m-%d')


def trim_string_ck3(string: str) -> str:
    string = (
        string
        .replace('dynn_', '')
        .replace('_lifestyle', '')
        .replace('_perk', '')
        .replace('nick_', '')
        .replace('death_', '')
        .replace('ethos_', '')
        .replace('house_', '')
        .replace('heritage_', '')
        .replace('martial_custom_', '')
        .replace('tradition_', '')
        .replace('fp2_', '')
        .replace('fp1_', '')
        .replace('language_', '')
        .replace('_1', '')
        .replace('_2', '')
        .replace('doctrine_', '')
        .replace('special_', '')
        .replace('is_', '')
        # .replace('A_', 'ã').replace('O_', 'õ').replace('E_', 'ẽ')
        .replace('_', ' ')
        .lower()
        .capitalize()
    )
    return string



def get_title_from_prefix(findKey: str, baseName: str) -> str:
    prefix_map = {
        'b_': 'Barony of ',
        'c_': 'County of ',
        'd_': 'Duchy of ',
        'k_': 'Kingdom of ',
        'e_': 'Empire of '
    }
    
    for prefix, title in prefix_map.items():
        if prefix in findKey:
            return title + baseName
    
    return baseName


def inject_title(rawData: str, findKey: str, baseName: str) -> str:
    if 'article' in rawData:
        findArt = re.findall(r'article="(.*?)"', rawData, re.S)
        return findArt[0] + baseName
    
    return get_title_from_prefix(findKey, baseName)

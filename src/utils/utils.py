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


def convert_ck3_date(date_str: str) -> str:
    """Convert CK3 date format (YYYY.M.D) to standard date format (YYYY-MM-DD)."""
    return datetime.strptime(date_str, "%Y.%m.%d")#.strftime("%Y-%m-%d")

import re
from datetime import datetime
from src.utils.utils import convert_ck3_date

# EXTRACT CHARACTER FROM CK3 DATA -------------------------------------------------------

def extract_character_from_id(charid:str, data:str) -> str:
    charData = re.findall(r'\n%s={\n\tfirst_name=.+?\n}' % charid, data, re.S)
    return charData[0]

# PARSER FOR CHARACTER FEATURES -------------------------------------------------------

def retrieve_character_info(id, data):
    """
    Retrieve detailed information about a character from the provided data.
    Args:
        id (int): The unique identifier of the character.
        data (str): The raw data containing character information.
    Returns:
        dict: A dictionary containing the character's information, including:
            - id (int): The unique identifier of the character.
            - name (str): The first name of the character.
            - nickname (str): The nickname of the character.
            - birth (datetime): The birth date of the character in standard datetime format.
            - dynasty (str): The dynasty house of the character.
            - culture (str): The culture of the character.
            - faith (str): The faith of the character.
            - skills (str): A comma-separated list of the character's skills.
            - traits (str): A comma-separated list of the character's traits.
            - recessive_traits (str): A comma-separated list of the character's recessive traits, or 'None' if none.
            - death (datetime): The death date of the character in standard datetime format, if applicable.
            - cause_of_death (str): The cause of death of the character, if applicable.
    Example:
        >>> data = '...raw character data...'
        >>> character_info = retrieve_character_info(12345, data)
        >>> print(character_info)
        {
            'id': 12345,
            'name': 'John',
            'nickname': 'The Brave',
            'birth': datetime.datetime(1066, 9, 15),
            'dynasty': 'House Stark',
            'culture': 'Northman',
            'faith': 'Old Gods',
            'skills': 'martial, stewardship',
            'traits': 'brave, diligent',
            'recessive_traits': 'none',
            'death': datetime.datetime(1100, 3, 1),
            'cause_of_death': 'natural causes'
    """

    charData = extract_character_from_id(id, data)
    findName = re.findall(r'first_name="(.*?)"', charData, re.S)
    nick_name = re.findall(r'nickname_text="(.*?)"', charData, re.S)[0]

    # Extract birth date and convert to standard datetime format
    birth = re.findall(r'birth=(.*?)\n', charData, re.S)
    birth_date = convert_ck3_date(birth[0])

    # Dynasty, culture & faith
    dynasty_character = re.findall(r'dynasty_house=(\d+)', charData)
    findCulture = re.findall(r'culture=(.*?)\n', charData, re.S)
    faith_character = re.findall(r'faith=(.*?)\n', charData)

    # Skills and traits
    skills_character = re.findall(r'skill={(.*?)}', charData, re.S)[0].split(' ')
    skills_character = [skill for skill in skills_character if skill != '']
    findTraits = re.findall(r'traits={(.*?)}', charData, re.S)[0].split(' ')[1:-1]
    findRecessive = re.findall(r'recessive_traits={(.*?)}', charData, re.S)
    if len(findRecessive) > 0:
        findRecessive = findRecessive[0].split(' ')[1:-1]

    character_info = {
        'id': id,
        'name': findName[0],
        'nickname': nick_name,
        'birth': birth_date,
        'dynasty': dynasty_character[0] if dynasty_character else 'Unknown', 
        'culture': findCulture[0] if findCulture else 'Unknown', 
        'faith': faith_character[0] if faith_character else 'Unknown',
        'skills': ', '.join(skills_character), 
        'traits': ', '.join(findTraits), 
        'recessive_traits': ', '.join(findRecessive) if findRecessive else 'None'
    }

    character_info_death = extract_death_data(charData)

    # Combine character_info and character_info_death dictionaries
    combined_info = {**character_info, **character_info_death}

    return combined_info


# PARSER FOR DEATH DATA -------------------------------------------------------

def extract_death_data(charData):
    """
    Extracts death-related data from a given character data string.
    Args:
        charData (str): The character data string containing death information.
    Returns:
        dict: A dictionary containing the extracted death data with the following keys:
            - death_date (str): The converted death date.
            - death_reason (str): The reason for death.
            - liege (str): The liege information.
            - liege_title (str): The titles of the liege, joined by commas.
            - domain (str): The domain information at the time of death, formatted with commas.
    """

    death_data = re.findall(r'dead_data={(.*?)}', charData, re.S)
    
    if not death_data:
        return {}
    death_data = death_data[0]

    # Convert death date
    death_date = re.findall(r'date=(.*?)\n', death_data, re.S)[0]
    death_date = convert_ck3_date(death_date)

    # Extract death reason
    death_reason = re.findall(r'reason=(.*?)\n', death_data, re.S)[0]

    # Extract liege information
    liege = re.findall(r'liege=(.*?)\n', death_data, re.S)[0]
    liege_title = re.findall(r'liege_title=(.*?)\n', death_data, re.S)
    liege_title = ", ".join(liege_title)

    # Extract domain
    domain_death = re.findall(r'domain={(.*?)}', death_data + "}", re.S) # Add closing bracket to avoid error
    domain_death = domain_death[0].strip().replace(" ", ", ")

    death_data_dict = { 
        "death_date": death_date,
        "death_reason": death_reason,
        "liege_at_death": liege,
        "liege_title_at_death": liege_title,
        "domain_at_death": domain_death,
    }

    return death_data_dict
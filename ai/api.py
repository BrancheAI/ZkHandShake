
import os
import openai
from prompt_input import prompt

openai.api_key = "<Your API key goes here>"

def process_gpt4(text):
    """This function prompts the gpt-4 model and returns the output"""

    response = openai.ChatCompletion.create(
        model="gpt-4",
        temperature=0,
        messages=[
            {"role": "user", "content": prompt + text},
        ],
    )

    result = response['choices'][0]['message']['content']

    return result


def process_gpt3(text):
    """This function prompts the gpt-3.5 model and returns the output"""

    response = openai.ChatCompletion.create(
        model="gpt-3",
        temperature=0,
        messages=[
            {"role": "user", "content": prompt + text},
        ],
    )

    result = response['choices'][0]['message']['content']

    return result

if __name__ == "__main__":
    print(process_gpt3(""))
from gift_suggestions_generator import exports
from gift_suggestions_generator.exports.generator import Suggestions
# from spin_sdk import llm

class Generator(exports.Generator):
    def suggest(self, name: str, age: int, likes: str):
        # Implement your gift suggestion here
        return Suggestions("Riley Parker", "Better Spin error messages!!!!")

import json
import sys


def remove_path_cost_and_left_category(input_json_path: str) -> str:
    new_lattice = {}
    input_json = json.load(open(input_json_path))

    new_lattice["sentence"] = input_json["sentence"]
    new_lattice["lattice"] = {"word_nodes": []}

    lattice = input_json["lattice"]
    for i, nodes in enumerate(lattice):
        word_node = []
        for j in range(len(nodes)):
            current_morph_node = lattice[i][j]
            morph_node = {}
            morph_node["word"] = {
                "base": current_morph_node["words"][0]["base"],
                "suffixes": current_morph_node["words"][0]["suffixes"],
                "part_of_speech": current_morph_node["words"][0]["part_of_speech"],
            }

            # 接語が連綴されている場合
            if not len(current_morph_node["words"]) == 1:
                morph_node["clitic"] = {
                    "clitic": current_morph_node["words"][1]["base"],
                    "case": current_morph_node["words"][1]["detail"],
                }
            word_node.append(morph_node)
        new_lattice["lattice"]["word_nodes"].append(word_node)
    input_json["lattice"] = lattice
    return json.dumps(new_lattice)


if __name__ == "__main__":
    input_json_path = sys.argv[1]
    print(remove_path_cost_and_left_category(input_json_path))

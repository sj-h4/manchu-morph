import json
import sys


def remove_path_cost_and_left_category(input_json: str) -> str:
    new_lattice = {}
    json_dict = json.loads(input_json)

    new_lattice["sentence"] = json_dict["sentence"]
    new_lattice["lattice"] = {"word_nodes": []}

    lattice = json_dict["lattice"]
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
    json_dict["lattice"] = lattice
    return json.dumps(new_lattice, ensure_ascii=False)


if __name__ == "__main__":
    input_json = input()
    print(remove_path_cost_and_left_category(input_json))

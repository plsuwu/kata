#!/usr/bin/env python


def dir_reduc(arr: list[str]) -> list[str]:
    print(f"INIT: {arr}")
    card = ["NORTH", "SOUTH", "EAST", "WEST"]
    coord = [0, 0]
    rm_card = [0, 0]

    for idx, i in enumerate(arr):
        print(f"enum @ {idx}: {i} ")
        match i:
            case "NORTH":
                coord[0] += 1

                if arr[idx - 1] == "SOUTH":
                    arr.remove("NORTH")
                    arr.remove("SOUTH")
                elif arr[idx - 1] == "EAST" and (coord == [0, 0]):
                    arr.remove("NORTH")
                    arr.remove("SOUTH")

            case "SOUTH":
                coord[0] -= 1

                if arr[idx - 1] == "NORTH":
                    arr.remove("NORTH")
                    arr.remove("SOUTH")
                elif arr[idx - 1] == "EAST" and (coord == [0, 0]):
                    arr.remove("NORTH")
                    arr.remove("SOUTH")

            case "EAST":
                coord[1] += 1

                if arr[idx - 1] == "WEST":
                    arr.remove("EAST")
                    arr.remove("WEST")
                elif arr[idx - 1] == "WEST" and (coord == [0, 0]):
                    arr.remove("EAST")
                    arr.remove("WEST")

            case "WEST":
                coord[1] -= 1

                if arr[idx - 1] == "EAST":
                    arr.remove("EAST")
                    arr.remove("WEST")
                elif arr[idx - 1] == "EAST" and (coord == [0, 0]):
                    arr.remove("EAST")
                    arr.remove("WEST")
        print(coord)

    print(f"{coord=}, {rm_card=}")
    for i, n in enumerate(rm_card):
        for idx in range(n):
            if i == 0:
                arr.remove("NORTH")
                arr.remove("SOUTH")
            else:
                arr.remove("EAST")
                arr.remove("WEST")

            print(f'removing {"EAST/WEST" if i != 0 else "NORTH/SOUTH"}')
            # if n
            # arr.remove("EAST")

    print(f"FINAL: {arr}")
    return [""]


print("\n\nT1 (expects '[W]')")
dir_reduc(["NORTH", "SOUTH", "SOUTH", "EAST", "WEST", "NORTH", "WEST"])

print("\n\nT2 (expects '[N N]')")
dir_reduc(
    [
        "NORTH",
        "SOUTH",
        "EAST",
        "WEST",
        "NORTH",
        "NORTH",
        "SOUTH",
        "NORTH",
        "WEST",
        "EAST",
    ]
)

print("\n\nT3 (expects '[N W S E]')")
dir_reduc(["NORTH", "WEST", "SOUTH", "EAST"])

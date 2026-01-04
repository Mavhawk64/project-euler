import os
from typing import List, Optional


class Node(object):
    def __init__(
        self, val: int, left: Optional["Node"] = None, right: Optional["Node"] = None
    ):
        self.val = val
        self.left = left
        self.right = right

    def __str__(self) -> str:
        return f"{self.val:04d}"

    def has_child(self) -> bool:
        return self.left is not None or self.right is not None


class Tree(object):
    root: Optional[Node] = None

    def __init__(self, structure: Optional[str] = None):
        if not structure:
            return
        # here we need to convert our str structure to a tree.
        rows = []
        # we'll first go row by row, adding nodes
        for line in structure.strip().split("\n"):
            if line.strip():
                row_nodes = [Node(int(val)) for val in line.split()]
                rows.append(row_nodes)
        if not rows:
            return
        for i in range(len(rows) - 1):
            for j in range(len(rows[i])):
                current_node = rows[i][j]
                # if you look at input.txt or triangle.txt (rather than the pyramid view, it is apparent that we should use same index for left, +1 for right)
                current_node.left = rows[i + 1][j]
                current_node.right = rows[i + 1][j + 1]
        # ok we should have all the rows of nodes with their children loaded in.
        # since the top of the file is always the "tree's root", we can just set_root(rows[0][0] ~ since it is 2-D array)
        self.set_root(rows[0][0])

    def set_root(self, root: Node):
        self.root = root

    def eat_level(self):
        # go to the max depth, and their roots will now be like this example:
        # Node(root=True, val = 3, left.val = 9, right.val = 7) -> self = Node(root=False, val = self.val + max(self.left.val,self.right.val))
        d = self.depth()
        if d <= 1:
            return (
                False,
                "Depth is equal to or less than 1. No levels to eat!",
            )  # just break out with failed attempt
        for node in self.get_level(d - 1):
            node.val = node.val + max(node.left.val, node.right.val)  # type: ignore -- all nodes will have left and right
            node.left = None
            node.right = None
        return True, "Level consumed."

    def get_level(self, depth) -> List[Node]:
        levels = []
        current_level = [self.root]

        # essentially copy this from the toString() method except instead of str(n) for n in ... we use n for n (whatever)
        while any(node is not None for node in current_level):
            levels.append([n for n in current_level])
            next_level = []

            for i, node in enumerate(current_level):
                if node:
                    next_level.append(node.left)
                    if i == len(current_level) - 1:
                        next_level.append(node.right)
                else:
                    next_level.extend([None])

            if all(n is None for n in next_level):
                break
            current_level = next_level

        # Now that we have all the levels, just access the depth level
        return levels[depth - 1]

    def depth(self) -> int:
        count: int = 1
        if not self.root:
            return 0
        curr_node: Node = self.root
        while curr_node.has_child():
            count += 1
            curr_node = curr_node.left  # type: ignore # Just go left since it is a nice tree.
        return count

    def __str__(self) -> str:
        """
        Clean str() for displaying the tree, courtesy of Gemini (Google AI).
        """
        if not self.root:
            return "Empty Tree"

        levels = []
        current_level = [self.root]

        while any(node is not None for node in current_level):
            # Use the Node's __str__ (which now pads zeros)
            levels.append([str(n) if n else "    " for n in current_level])
            next_level = []

            for i, node in enumerate(current_level):
                if node:
                    next_level.append(node.left)
                    if i == len(current_level) - 1:
                        next_level.append(node.right)
                else:
                    next_level.extend([None])

            if all(n is None for n in next_level):
                break
            current_level = next_level

        # Increase width and spacing because each number is now 4 chars wide
        width = len(levels[-1]) * 8
        output = []
        for level in levels:
            # Join with more spaces to keep the triangle shape clean
            level_str = "    ".join(level)
            output.append(level_str.center(width))

        return "\n".join(output)


# Execution
# INPUT = "03\n07 04\n02 04 06\n08 05 09 03"

with open(
    os.path.join(os.path.dirname(__file__), "input.txt"), "r", encoding="utf-8"
) as f:
    INPUT = f.read()

t = Tree(INPUT)
print(t)

print("\n\n\n\n")

# print(t.depth())
# print([x.val for x in t.get_level(14)])
while t.eat_level()[0]:
    print(t)
    print("\n\n\n\n")

print(t.root.val)  # type: ignore

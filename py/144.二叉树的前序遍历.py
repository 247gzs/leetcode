# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def preorderTraversal(self, root: TreeNode) -> List[int]:
        result = []
        self.dfs(root, result)
        return result

    def dfs(self, root, result):
        if not root:
            return 
        result.append(root.val)
        if root.left:
            self.dfs(root.left, result)
        if root.right:
            self.dfs(root.right, result)

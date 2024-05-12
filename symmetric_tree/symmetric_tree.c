#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

typedef struct TreeNode
{
    int val;
    struct TreeNode *left;
    struct TreeNode *right;
} TreeNode;

bool sym(struct TreeNode* leftNode, struct TreeNode* rightNode)
{
    if (leftNode == NULL || rightNode == NULL)
    {
        return leftNode == rightNode;
    }

    return (leftNode->val == rightNode->val)
            && (sym(leftNode->left, rightNode->right))
            && (sym(leftNode->right, rightNode->left));
}

bool isSymmetric(struct TreeNode* root)
{
    return sym(root, root);
}

int main()
{
    TreeNode* root = (TreeNode*)malloc(sizeof(TreeNode));
    root->val = 1;
    TreeNode* l = (TreeNode*)malloc(sizeof(TreeNode));
    l->val = 2;
    TreeNode* r = (TreeNode*)malloc(sizeof(TreeNode));
    r->val = 2;
    TreeNode* ll = (TreeNode*)malloc(sizeof(TreeNode));
    ll->val = 3;
    TreeNode* lr = (TreeNode*)malloc(sizeof(TreeNode));
    lr->val = 4;
    TreeNode* rl = (TreeNode*)malloc(sizeof(TreeNode));
    rl->val = 4;
    TreeNode* rr = (TreeNode*)malloc(sizeof(TreeNode));
    rr->val = 3;

    root->left = l;
    root->right = r;
    l->left = ll;
    l->right = lr;
    r->left = rl;
    r->right = rr;
    ll->left = NULL;
    ll->right = NULL;
    lr->right = NULL;
    lr->left = NULL;
    rl->left = NULL;
    rl->right = NULL;
    rr->left = NULL;
    rr->right = NULL;

    if (isSymmetric(root))
    {
        printf("Tree is symmetric\n");
    }
    else
    {
        printf("Tree is not symmetric\n");
    }

    free(root);
    free(r);
    free(l);
    free(lr);
    free(ll);
    free(rl);
    free(rr);

    return 0;
}
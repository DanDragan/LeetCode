#include <iostream>
#include <vector>

class Solution {
public:
    void rotate(std::vector<std::vector<int>>& matrix) {
        unsigned int size = matrix[0].size();

        for (auto i = 0; i < size; i++)
        {
            for (auto j = i + 1; j < size; j++)
            {
                int tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }

        for (auto i = 0; i < size; i++)
        {
            for (auto j = 0; j < size / 2; j++)
            {
                int tmp = matrix[i][j];
                matrix[i][j] = matrix[i][size - j - 1];
                matrix[i][size - j - 1] = tmp;
            }
        }
    }
};

int main(void)
{
    Solution *sol = new Solution();
    std::vector<std::vector<int>> matrix;
    std::vector<int> row1 = {1, 2, 3};
    std::vector<int> row2 = {4, 5, 6};
    std::vector<int> row3 = {7, 8, 9};
    matrix.push_back(row1);
    matrix.push_back(row2);
    matrix.push_back(row3);

    sol->rotate(matrix);

    for (auto i = 0; i < matrix[0].size(); i++)
    {
        for (auto j:matrix[i])
        {
            std::cout << j << " ";
        }

        std::cout << std::endl;
    }

    delete sol;

    return 0;
}
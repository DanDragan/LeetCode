#include <iostream>

int uniquePaths(int m, int n) {
    if (m == 1 || n == 1)
    {
        return 1;
    }

    unsigned int** matrix = new unsigned int* [m];

    unsigned int** matrix = new unsigned int* [num_rows];

    for (auto i = 0; i < num_rows; i++)
    {
        matrix[i] = new unsigned int [n];
    }

    for (auto i = 0; i < n; i++)
    {
        matrix[0][i] = 1;
    }

    for (auto i = 0; i < num_rows; i++)
    {
        matrix[i][0] = 1;
    }

    for (auto i = 1; i < m; i++)
    {
        for (auto j = 1; j < n; j++)
        {
            if (i % 2)
            {
                matrix[1][j] = matrix[0][j] + matrix[1][j-1];
            }
            else
            {
                matrix[0][j] = matrix[1][j] + matrix[0][j-1];
            }
        }
    }

    return std::max(matrix[0][n-1], matrix[1][n-1]);            
}

int main(int argc, char* argv[])
{
    std::cout << "Unique paths: " << uniquePaths(3, 7) << std::endl;
}
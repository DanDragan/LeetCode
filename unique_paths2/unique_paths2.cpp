#include <iostream>
#include <vector>

int uniquePathsWithObstacles(std::vector<std::vector<int>>& obstacleGrid) {
    int set = 1;
    unsigned int m = obstacleGrid.size();
    unsigned int n = obstacleGrid[0].size();

    if (obstacleGrid[0][0] == 1)
    {
        return 0;
    }

    for (auto i = 0; i < n; i++)
    {
        if (obstacleGrid[0][i] == 1) {
            set = 0;
        }

        obstacleGrid[0][i] = set;
    }

    if (m == 1)
    {
        if (set == 0)
        {
            return 0;
        }
        else
        {
            return 1;
        }
    }

    set = 1;

    for (auto i = 1; i < m; i++)
    {
        if (obstacleGrid[i][0] == 1) {
            set = 0;
        }

        obstacleGrid[i][0] = set;
    }

    if (n == 1)
    {
        if (set == 0)
        {
            return 0;
        }
        else
        {
            return 1;
        }
    }

    for (auto i = 1; i < m; i++)
    {
        for (auto j = 1; j < n; j++)
        {
            if (obstacleGrid[i][j] == 1)
            {
                obstacleGrid[i][j] = 0;
            }
            else
            {
                obstacleGrid[i][j] = obstacleGrid[i-1][j] + obstacleGrid[i][j-1];
            }
        }
    }

    return obstacleGrid[m-1][n-1];
}

int main(void)
{
    std::vector<std::vector<int>> obstacleGrid;
    std::vector<int> row1;
    std::vector<int> row2;
    std::vector<int> row3;
    row1.push_back(0);
    row1.push_back(0);
    row1.push_back(0);
    obstacleGrid.push_back(row1);
    row2.push_back(0);
    row2.push_back(1);
    row2.push_back(0);
    obstacleGrid.push_back(row2);
    row3.push_back(0);
    row3.push_back(0);
    row3.push_back(0);
    obstacleGrid.push_back(row3);
    std::cout << "Unique paths: " << uniquePathsWithObstacles(obstacleGrid) << std::endl;
}
#include <iostream>
#include <vector>
#include <unordered_map>

class Solution {
public:
    int fourSumCount(std::vector<int>& nums1, std::vector<int>& nums2, std::vector<int>& nums3, std::vector<int>& nums4) {
        std::unordered_map<int, int> u_map;
        unsigned int count = 0;

        for (auto i : nums1)
        {
            for (auto j : nums2)
            {
                ++u_map[i + j];
            }
        }

        for (auto i : nums3)
        {
            for (auto j : nums4)
            {
                int neg_sum = -(i + j);
                if (u_map.contains(neg_sum))
                {
                    count += u_map[neg_sum];
                }
            }
        }

        return count;
    }
};

int main(void)
{
    Solution *sol = new Solution();
    std::vector<int> nums1 = {1, 2};
    std::vector<int> nums2 = {-2, -1};
    std::vector<int> nums3 = {-1, 2};
    std::vector<int> nums4 = {0, 2};

    std::cout << "Result is: " << sol->fourSumCount(nums1, nums2, nums3, nums4) << std::endl;

    std::vector<int> nums5 = {-1, -1};
    std::vector<int> nums6 = {-1, 1};
    std::vector<int> nums7 = {-1, 1};
    std::vector<int> nums8 = {1, -1};

    std::cout << "Result is: " << sol->fourSumCount(nums5, nums6, nums7, nums8) << std::endl;

    delete sol;

    return 0;
}
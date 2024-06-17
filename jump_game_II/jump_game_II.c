#include <stdio.h>

int jump(int* nums, int numsSize) {

    int jumps = 0;
    int last = 0;
    int farthest = 0;

    for (int i = 0; i < numsSize - 1; ++i)
    {
        if (farthest < nums[i] + i)
        {
            farthest = nums[i] + i;
        }

        if (farthest >= numsSize - 1) {
            jumps++;
            break;
        }

        if (i == last)
        {
            jumps++;
            last = farthest;
        }
    }

    return jumps;
}

int main(void)
{

    int nums1[] = {2, 3, 1, 1, 4};
    int numSize1 = 5;
    int nums2[] = {2, 3, 0, 1, 4};
    int numSize2 = 5;
    int nums3[] = {0};
    int numSize3 = 1;
    int nums4[] = {2, 1};
    int numSize4 = 2;
    int nums5[] = {1, 2, 3};
    int numSize5 = 3;

    printf("Num jumps: %d\n", jump(nums1, numSize1));
    printf("Num jumps: %d\n", jump(nums2, numSize2));
    printf("Num jumps: %d\n", jump(nums3, numSize3));
    printf("Num jumps: %d\n", jump(nums4, numSize4));
    printf("Num jumps: %d\n", jump(nums5, numSize5));
}
#include <cstdio>
#include <regex>
#include <fstream>
#include <iostream>
#include <string>

std::string read_file(const char* filename)
{
    // https://stackoverflow.com/questions/116038/how-do-i-read-an-entire-file-into-a-stdstring-in-c
    std::ifstream in;
    std::ostringstream sstr;
    in.open("input.txt");
    sstr << in.rdbuf();
    in.close();
    return sstr.str();
}

void add_valid_muls(int& result, std::string s)
{
    std::regex re("mul\\(\\d+,\\d+\\)");
    auto matches_begin = std::sregex_iterator(s.begin(), s.end(), re);
    auto matches_end = std::sregex_iterator();
    for (auto match_i = matches_begin; match_i != matches_end; ++match_i) {
        std::smatch m = *match_i;
        std::string match = m.str();
        match.erase(match.begin(), match.begin() + 4);
        match.pop_back();
        size_t comma_index = match.find_first_of(",");
        int x1 = std::stoi(match.substr(0, comma_index));
        int x2 = std::stoi(match.substr(comma_index + 1, match.length()));
        result += x1 * x2;
    }
}

void part_1()
{
    int result = 0;
    std::string input = read_file("input.txt");
    std::regex re("mul\\(\\d+,\\d+\\)");
    // https://en.cppreference.com/w/cpp/regex
    add_valid_muls(result, input);
    printf("Sum of all valid mul operations = %d\n", result);
}

void part_2()
{
    int result = 0;
    std::string input = read_file("input.txt");
    size_t do_index = 0;
    size_t dont_index = input.find("don't()");
    add_valid_muls(result, input.substr(0, dont_index + 7));
    while (true) {
        do_index = input.find("do()", dont_index);
        dont_index = input.find("don't()", do_index);
        if (do_index == std::string::npos) break;
        add_valid_muls(result, input.substr(do_index, dont_index - do_index + 7));
    }
    printf("Sum of all valid mul operations between dos and don'ts= %d\n", result);
    return;
}

int main()
{
    part_1();
    part_2();
    return 0;
}
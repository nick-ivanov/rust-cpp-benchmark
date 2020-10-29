#include <algorithm>
#include <ctime>
#include <fstream>
#include <functional>
#include <iostream>
#include <random>
#include <string>
#include <vector>
#include <cstring>
#include <chrono>
#include <map>
#include <unordered_map>
#include <cassert>
using namespace std;

// This function is inspired by the code originally written
// by rep_movsd and published here:
// https://stackoverflow.com/questions/2704521/generate-random-double-numbers-in-c
double gen_rand_double_range(double mn, double mx)
{
    return mn + ((double) rand() / RAND_MAX) * (mx - mn);
}

// This function is inspired by the code originally written
// by rep_movsd and published here:
// https://stackoverflow.com/questions/2704521/generate-random-double-numbers-in-c
uint64_t gen_rand_long_range(uint64_t mn, uint64_t mx)
{
    return mn + ((uint64_t) rand() / RAND_MAX) * (mx - mn);
}

// This function is inspired by the code originally written 
// by Dr. Charles Ofria and published here:
// https://raw.githubusercontent.com/mercere99/CSE-830/master/InClass/Week2/TestSorts.cc
void insertion_sort(vector<double> & v, int start = 0, int end = -1)
{
    if (end == -1) {
        end = v.size();
    }

    for (int i = start+1; i < end; i++) {
        double key = v[i];
        int j = i - 1;

        while (j >= start && v[j] > key) {
            v[j+1] = v[j];
            --j;
        }

        v[j+1] = key;
    }
}

// This function is inspired by the code originally written 
// by Dr. Charles Ofria and published here:
// https://raw.githubusercontent.com/mercere99/CSE-830/master/InClass/Week2/TestSorts.cc
void merge(vector<double> & v, int start, int midpoint, int end) {
    vector<double> merged;
    int p1 = start;
    int p2 = midpoint;

    while (p1 < midpoint && p2 < end) {
        if (v[p1] < v[p2]) { merged.push_back(v[p1++]); }
        else { merged.push_back(v[p2++]); }
    }

    while (p1 < midpoint) merged.push_back(v[p1++]);
    while (p2 < end) merged.push_back(v[p2++]);

    copy_n(merged.begin(), merged.size(), v.begin()+start);
}

// This function is inspired by the code originally written 
// by Dr. Charles Ofria and published here:
// https://raw.githubusercontent.com/mercere99/CSE-830/master/InClass/Week2/TestSorts.cc
void merge_sort(vector<double> & v, int start=0, int end=-1)
{
    if (end == -1) end = v.size();

    if (end - start < 2) return;

    int midpoint = (start + end) / 2;
    merge_sort(v, start, midpoint);
    merge_sort(v, midpoint, end);
    merge(v, start, midpoint, end);
}

// This function is inspired by the code originally written 
// by Dr. Charles Ofria and published here:
// https://raw.githubusercontent.com/mercere99/CSE-830/master/InClass/Week2/TestSorts.cc
void hybrid_sort(vector<double> & v, int start=0, int end=-1, int k = 0)
{
    if (end == -1) end = v.size();

    if (end - start < k) {
        insertion_sort(v, start, end);
        return;
    }

    int midpoint = (start + end) / 2;
    hybrid_sort(v, start, midpoint, k);
    hybrid_sort(v, midpoint, end, k);
    merge(v, start, midpoint, end);
}

void hashmap_insert(vector<uint64_t> & keys, vector<double> & values, unordered_multimap<uint64_t,double> & hashmap) {
    assert(keys.size() == values.size());

    for(int i = 0; i < keys.size(); i++) {
        hashmap.insert(make_pair(keys[i], values[i]));
    }
}

void hashmap_delete(vector<uint64_t> & keys, unordered_multimap<uint64_t,double> & hashmap) {
    for(int i = 0; i < keys.size(); i++) {
        hashmap.erase(keys[i]);
    }
}

void bt_insert(vector<uint64_t> & keys, vector<double> & values, multimap<uint64_t,double> & bintree) {
    assert(keys.size() == values.size());

    for(int i = 0; i < keys.size(); i++) {
        bintree.insert(make_pair(keys[i], values[i]));
    }
}

void bt_delete(vector<uint64_t> & keys, multimap<uint64_t,double> & bintree) {
    for(int i = 0; i < keys.size(); i++) {
        bintree.erase(keys[i]);
    }
}


void print_usage() {
    cout << "USAGE:" << endl;
    // cout << "csort <merge|insert> <vector_size> <number_of_runs>" << endl;
    // cout << "csort hybrid <threshold> <vector_size> <number_of_runs>" << endl;

    cout << "csort merge <vector_size> <number_of_runs>" << endl;
    cout << "csort insert <vector_size> <number_of_runs>" << endl;
    cout << "csort hybrid <threshold> <vector_size> <number_of_runs>" << endl;
    cout << "csort hashmap <vector_size> <number_of_runs>" << endl;
    cout << "csort bintree <vector_size> <number_of_runs>" << endl;
}

int main(int argc, char *argv[])
{
    int hybrid_threshold = 200, number_of_runs = 10000;
    
    if(argc < 2) {
        print_usage();
        return 1;
    }

    if(strcmp(argv[1], "merge") && strcmp(argv[1], "insert") &&
    strcmp(argv[1], "hashmap") && strcmp(argv[1], "bintree") && strcmp(argv[1], "hybrid") ) {
        print_usage();
        return 1;
    }

    assert(number_of_runs >= 1);
    double sumruns = 0.0;
    double sumruns1 = 0.0;

    vector<double> res;
    vector<double> res1;

    if(!strcmp(argv[1], "merge")) {
        for(int vector_size = 25; vector_size <= 1000; vector_size += 25) {
            sumruns = 0;
            sumruns1 = 0;
            for(int i = 0; i < number_of_runs; i++) {
                vector<double> v;
                v.reserve(vector_size);

                for(int j = 0; j < vector_size; j++) {
                    v.push_back(gen_rand_double_range(1, 65536));
                }

                // The following time measurement routine is suggested by
                // Dr. Charles Ofria in CSE830 Homework 4 assignment (Fall 2020)
                std::clock_t start_time = std::clock();

                merge_sort(v, 0, vector_size-1);

                std::clock_t tot_time = std::clock() - start_time;
    
                sumruns += (((double) tot_time) / (double) CLOCKS_PER_SEC);
            }

            cout << fixed << (sumruns / number_of_runs) << ", " << flush;
        }
    }

    if(!strcmp(argv[1], "insert")) {
        for(int vector_size = 25; vector_size <= 1000; vector_size += 25) {
            sumruns = 0;
            sumruns1 = 0;
            for(int i = 0; i < number_of_runs; i++) {
                vector<double> v;
                v.reserve(vector_size);

                for(int j = 0; j < vector_size; j++) {
                    v.push_back(gen_rand_double_range(1, 65536));
                }

                // The following time measurement routine is suggested by
                // Dr. Charles Ofria in CSE830 Homework 4 assignment (Fall 2020)
                std::clock_t start_time = std::clock();

                insertion_sort(v, 0, vector_size-1);

                std::clock_t tot_time = std::clock() - start_time;

                sumruns += (((double) tot_time) / (double) CLOCKS_PER_SEC);
            }

            cout << fixed << (sumruns / number_of_runs) << ", " << flush;
        }
    }

    if(!strcmp(argv[1], "hybrid")) {
        hybrid_threshold = 16;
        for(int vector_size = 250; vector_size <= 10000; vector_size += 250) {
            sumruns = 0;
            sumruns1 = 0;
            for(int i = 0; i < number_of_runs; i++) {
                vector<double> v;
                v.reserve(vector_size);

                for(int j = 0; j < vector_size; j++) {
                    v.push_back(gen_rand_double_range(1, 65536));
                }

                // The following time measurement routine is suggested by
                // Dr. Charles Ofria in CSE830 Homework 4 assignment (Fall 2020)
                std::clock_t start_time = std::clock();

                hybrid_sort(v, 0, vector_size-1, hybrid_threshold);

                std::clock_t tot_time = std::clock() - start_time;

                sumruns += (((double) tot_time) / (double) CLOCKS_PER_SEC);
            }

            cout << fixed << (sumruns / number_of_runs) << ", " << flush;
        }
    }

    if(!strcmp(argv[1], "hashmap")) {
        for(int vector_size = 100; vector_size <= 10000; vector_size += 100) {
            cout << "running vector size " << vector_size << endl;
            sumruns = 0;
            sumruns1 = 0;
            for(int i = 0; i < number_of_runs; i++) {
                vector<uint64_t> keys;
                vector<double> values;
                unordered_multimap<uint64_t,double> hashmap;

                keys.reserve(vector_size);
                values.reserve(vector_size);

                for(int j = 0; j < vector_size; j++) {
                    keys.push_back(gen_rand_long_range(1, 18446744073709551615ULL));
                    values.push_back(gen_rand_double_range(1, 65536));
                }

                // The following time measurement routine is suggested by
                // Dr. Charles Ofria in CSE830 Homework 4 assignment (Fall 2020)
                std::clock_t start_time = std::clock();

                hashmap_insert(keys, values, hashmap);

                std::clock_t tot_time = std::clock() - start_time;
        
                sumruns += (((double) tot_time) / (double) CLOCKS_PER_SEC);

                // The following time measurement routine is suggested by
                // Dr. Charles Ofria in CSE830 Homework 4 assignment (Fall 2020)
                start_time = std::clock();

                hashmap_delete(keys, hashmap);

                tot_time = std::clock() - start_time;
        
                sumruns1 += (((double) tot_time) / (double) CLOCKS_PER_SEC);
            }

            res.push_back(sumruns);
            res1.push_back(sumruns1);
        }

        for(double x : res) {
            cout << fixed << (x / number_of_runs) << ", ";
        }

        cout << "\n---\n";

        for(double x : res1) {
            cout << fixed << (x / number_of_runs) << ", ";
        }
    }

    if(!strcmp(argv[1], "bintree")) {
        for(int vector_size = 100; vector_size <= 10000; vector_size += 100) {
            sumruns = 0;
            sumruns1 = 0;
            cout << "running vector size " << vector_size << endl;
            for(int i = 0; i < number_of_runs; i++) {
                vector<uint64_t> keys;
                vector<double> values;
                multimap<uint64_t,double> bintree;

                keys.reserve(vector_size);
                values.reserve(vector_size);

                for(int j = 0; j < vector_size; j++) {
                    keys.push_back(gen_rand_long_range(1, 18446744073709551615ULL));
                    values.push_back(gen_rand_double_range(1, 65536));
                }

                // The following time measurement routine is suggested by
                // Dr. Charles Ofria in CSE830 Homework 4 assignment (Fall 2020)
                std::clock_t start_time = std::clock();

                bt_insert(keys, values, bintree);

                std::clock_t tot_time = std::clock() - start_time;
        
                sumruns += (((double) tot_time) / (double) CLOCKS_PER_SEC);

                // The following time measurement routine is suggested by
                // Dr. Charles Ofria in CSE830 Homework 4 assignment (Fall 2020)
                start_time = std::clock();

                bt_delete(keys, bintree);

                tot_time = std::clock() - start_time;
        
                sumruns1 += (((double) tot_time) / (double) CLOCKS_PER_SEC);
            }

            res.push_back(sumruns);
            res1.push_back(sumruns1);
        }

        for(double x : res) {
            cout << fixed << (x / number_of_runs) << ", ";
        }

        cout << "\n---\n";

        for(double x : res1) {
            cout << fixed << (x / number_of_runs) << ", ";
        }
    }

    // cout << "AVERAGE: " << fixed << (sumruns / number_of_runs) << endl;

    // if(!strcmp(argv[1], "hashmap") || !strcmp(argv[1], "bintree")) {
    //     cout << "AVERAGE1: " << fixed << (sumruns1 / number_of_runs) << endl;
    // }

    return 0;
}
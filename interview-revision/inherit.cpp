#include <iostream>
#include <string>
#include <vector>

class Page {
    int word_count;
    int page_num;
    std::vector<std::string> words;
public:
    void addWord(std::string &words) {
        //std::vector insert method
    }

    virtual int totalWords() { return word_count };
};

class Book : Page {
    // In Book, page_num    stores the last visited page
    //          word_count  stores the total words from all pages
    std::vector<Page> pages;

    Book(std::vector<Page> &pages) : pages(pages) { }

    void insertPage(Page &page) {
        //std::vector to insert pages
    }

    std::vector::iterator insertPage(Page &page, int pos) {
        //std::vector insert in position
    }

    void getWordCount() { return this.word_count; }

    void getPageNumber() { return this.page_num; }

};

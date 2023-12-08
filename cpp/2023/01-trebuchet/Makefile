.PHONY: all
CXX = g++
LDLIBS = -lstdc++ $(shell pkg-config -libs fmt)
CXXFLAGS = -O3 -g -std=c++23 -Wall -Wextra -Wshadow -Wconversion -Wcast-align -Weffc++ -pedantic $(shell pkg-config -cflags fmt)

all: trebuchet

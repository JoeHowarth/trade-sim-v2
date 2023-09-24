# What Makes a Great Development Workflow?

Principles:
- Can build new functionality in isolation
    - unit tests for sim code
    - storybook for ui 
    - stored sample data for analysis
    - replay exact instant for debugging sim code 
- Create scenarios in python and save them to disk
- Python harness to 
    - run multiple scenarios at the same time 
    - run same scenario with different code 
- Python analytics to understand 
    - 1 run
    - 2 runs with same scneario, different code 
    - 2 runs with different scenario, same code
- Web Visualization 
    - Use url paths to quickly navigate to state of viewer
    - Choose output to load 
    - Explore output forward and backward
    - Most analytics are powered by python routes 
    - Input format viewer 
        - [Stretch] and builder / extender


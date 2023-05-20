# Classification and Regression Trees
Classification and Regression Trees or CART for short is an acronym introduced by Leo Breiman to refer to Decision Tree algorithms that can be used for classification or regression predictive modeling problems.

We will focus on using CART for classification in this tutorial.

The representation of the CART model is a binary tree. This is the same binary tree from algorithms and data structures, nothing too fancy (each node can have zero, one or two child nodes).

A node represents a single input variable (X) and a split point on that variable, assuming the variable is numeric. The leaf nodes (also called terminal nodes) of the tree contain an output variable (y) which is used to make a prediction.

Once created, a tree can be navigated with a new row of data following each branch with the splits until a final prediction is made.

Creating a binary decision tree is actually a process of dividing up the input space. A greedy approach is used to divide the space called recursive binary splitting. This is a numerical procedure where all the values are lined up and different split points are tried and tested using a cost function.

The split with the best cost (lowest cost because we minimize cost) is selected. All input variables and all possible split points are evaluated and chosen in a greedy manner based on the cost function.

Regression: The cost function that is minimized to choose split points is the sum squared error across all training samples that fall within the rectangle.
Classification: The Gini cost function is used which provides an indication of how pure the nodes are, where node purity refers to how mixed the training data assigned to each node is.
Splitting continues until nodes contain a minimum number of training examples or a maximum tree depth is reached.

Références :
* https://towardsdatascience.com/decision-tree-from-scratch-in-python-46e99dfea775
* https://anderfernandez.com/en/blog/code-decision-tree-python-from-scratch/
* https://machinelearningmastery.com/implement-decision-tree-algorithm-scratch-python/
* https://towardsdatascience.com/the-simple-math-behind-3-decision-tree-splitting-criterions-85d4de2a75fe
* https://www.cs.upc.edu/~mmartin/ml-mds/ml-Decision%20trees.pdf

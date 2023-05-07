# DS210-final-project
This is my final project for DS210

Writeup:

This project follows the model of "How often are friends of my friends my friends?" as an example. It uses 'facebook.txt,' a file that contains an undirected graph with ~4000 nodes. Each node represents a person, and their connection is another node that is their "friend." 

This code reads the facebook.txt (which is included in the repository but can also be found in the link below) and places it all into a vector of vectors where the index of each vector contains a vector with its connecting points on the graph. So if it was given: "0 1, 0 2, 1 2," the 0th index would contain a vector of 1,2, and the 1st index would have a vector of 2. 

The vector's graphs vector is then passed into the Jaccard similarity function module. This module calculates the Jaccard similarity between every vertex in the graph. The Jaccard similarity calculates what two lists have in common by taking the intersection (what each vector shares) and the union (their differences) and dividing them to create the Jaccard similarity. First, the function takes in the vector of vectors and iterates through each vector, using the idx of each vector to determine the exact vertex it is operating. Next, the function creates a HashSet for the vertex it is currently comparing and then iterates through the rest of the graph (skipping itself). Each iteration of this nested loop appends the two vertices being compared and their similarity score to a list. Once every vertex has been calculated, the function returns the list. 

This list is then sorted using a quicksort algorithm, and finally, the least similar and most similar points are printed.  

Example output:

the two LEAST similar points are 2674 and 3359 with a similarity of 0
the two MOST similar points are 3523 and 3481 with a similarity of 0.9166667



reference links:

Facebook dataset: https://snap.stanford.edu/data/ego-Facebook.html

Jaccard similarity: https://www.learndatasci.com/glossary/jaccard-similarity/



How to run this code:

download to desired location

open in desired editor

change directory to final_project

and run using cargo run --release 

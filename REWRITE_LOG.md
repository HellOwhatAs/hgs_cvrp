## User (checkpoint)

请将这个 C++ 实现的 HGS-CVRP 项目在 Rust 中重新实现，注意以下几点
1. 模块化设计与好的抽象，以获得良好的拓展性，但也不要过度抽象
2. 保持英文注释与可读性，简单明了即可，不必为了写注释而写
3. 零成本抽象的实践：尽可能避免 clone 或者 Rc/Arc 等不够 rust 的有开销实现
4. 与原始C++项目行为上基本一致（允许由于随机数生成器等语言本身因素造成的区别），但是效果应该基本一致

## Assistant

<details>
<summary>思考过程</summary>

I'll start by examining the project structure to get a sense of what we're working with.

</details>

我先来了解一下这个 C++ 项目的结构。

<details>
<summary>Tool Call: List the `HGS-CVRP` directory's contents</summary>

*Status: Completed*

# Folders:
HGS-CVRP/.github
HGS-CVRP/Instances
HGS-CVRP/Program
HGS-CVRP/Test

# Files:
HGS-CVRP/.gitignore
HGS-CVRP/CMakeLists.txt
HGS-CVRP/LICENSE
HGS-CVRP/README.md

</details>
<details>
<summary>Tool Call: List the `HGS-CVRP/Program` directory's contents</summary>

*Status: Completed*

# Files:
HGS-CVRP/Program/AlgorithmParameters.cpp
HGS-CVRP/Program/AlgorithmParameters.h
HGS-CVRP/Program/C_Interface.cpp
HGS-CVRP/Program/C_Interface.h
HGS-CVRP/Program/CircleSector.h
HGS-CVRP/Program/Genetic.cpp
HGS-CVRP/Program/Genetic.h
HGS-CVRP/Program/Individual.cpp
HGS-CVRP/Program/Individual.h
HGS-CVRP/Program/InstanceCVRPLIB.cpp
HGS-CVRP/Program/InstanceCVRPLIB.h
HGS-CVRP/Program/LocalSearch.cpp
HGS-CVRP/Program/LocalSearch.h
HGS-CVRP/Program/Params.cpp
HGS-CVRP/Program/Params.h
HGS-CVRP/Program/Population.cpp
HGS-CVRP/Program/Population.h
HGS-CVRP/Program/Split.cpp
HGS-CVRP/Program/Split.h
HGS-CVRP/Program/commandline.h
HGS-CVRP/Program/main.cpp

</details>
<details>
<summary>Tool Call: Read file `README.md`</summary>

*Status: Completed*

````HGS-CVRP/README.md
     1	
     2	[![CI_Build](https://github.com/vidalt/HGS-CVRP/actions/workflows/CI_Build.yml/badge.svg?branch=main)](https://github.com/vidalt/HGS-CVRP/actions/workflows/CI_Build.yml)
     3	
     4	# HGS-CVRP: A modern implementation of the Hybrid Genetic Search for the CVRP
     5	
     6	This is a modern implementation of the Hybrid Genetic Search (HGS) with Advanced Diversity Control of [1], specialized to the Capacitated Vehicle Routing Problem (CVRP).
     7	
     8	This algorithm has been designed to be transparent, specialized, and highly concise, retaining only the core elements that make this method successful.
     9	Beyond a simple reimplementation of the original algorithm, this code also includes speed-up strategies and methodological improvements learned over the past decade of research and dedicated to the CVRP.
    10	In particular, it relies on an additional neighborhood called SWAP*, which consists in exchanging two customers between different routes without an insertion in place.
    11	
    12	## References
    13	
    14	When using this algorithm (or part of it) in derived academic studies, please refer to the following works:
    15	
    16	[1] Vidal, T., Crainic, T. G., Gendreau, M., Lahrichi, N., Rei, W. (2012). 
    17	A hybrid genetic algorithm for multidepot and periodic vehicle routing problems. Operations Research, 60(3), 611-624. 
    18	https://doi.org/10.1287/opre.1120.1048 (Available [HERE](https://w1.cirrelt.ca/~vidalt/papers/HGS-CIRRELT-2011.pdf) in technical report form).
    19	
    20	[2] Vidal, T. (2022). Hybrid genetic search for the CVRP: Open-source implementation and SWAP* neighborhood. Computers & Operations Research, 140, 105643.
    21	https://doi.org/10.1016/j.cor.2021.105643 (Available [HERE](https://arxiv.org/abs/2012.10384) in technical report form).
    22	
    23	We also recommend referring to the Github version of the code used, as future versions may achieve better performance as the code evolves.
    24	The version associated with the results presented in [2] is [v1.0.0](https://github.com/vidalt/HGS-CVRP/releases/tag/v1.0.0).
    25	
    26	## Other programming languages
    27	
    28	There exist wrappers for this code in the following languages:
    29	* **C**: The **C_Interface** file contains a simple C API
    30	* **Python**: The [PyHygese](https://github.com/chkwon/PyHygese) package is maintained to interact with the latest release of this algorithm
    31	* **Julia**: The [Hygese.jl](https://github.com/chkwon/Hygese.jl) package is maintained to interact with the latest release of this algorithm
    32	
    33	We encourage you to consider using these wrappers in your different projects.
    34	Please contact me if you wish to list other wrappers and interfaces in this section.
    35	
    36	## Scope
    37	
    38	This code has been designed to solve the "canonical" Capacitated Vehicle Routing Problem (CVRP).
    39	It can also directly handle asymmetric distances as well as duration constraints.
    40	
    41	This code version has been designed and calibrated for medium-scale instances with up to 1,000 customers. 
    42	It is **not** designed in its current form to run very-large scale instances (e.g., with over 5,000 customers), as this requires additional solution strategies (e.g., decompositions and additional neighborhood limitations).
    43	If you need to solve problems outside of this algorithm's scope, do not hesitate to contact me at <thibaut.vidal@polymtl.ca>.
    44	
    45	## Compiling the executable 
    46	
    47	You need [`CMake`](https://cmake.org) to compile.
    48	
    49	Build with:
    50	```console
    51	mkdir build
    52	cd build
    53	cmake .. -DCMAKE_BUILD_TYPE=Release -G "Unix Makefiles"
    54	make bin
    55	```
    56	This will generate the executable file `hgs` in the `build` directory.
    57	
    58	Test with:
    59	```console
    60	ctest -R bin --verbose
    61	```
    62	
    63	## Running the algorithm
    64	
    65	After building the executable, try an example: 
    66	```console
    67	./hgs ../Instances/CVRP/X-n157-k13.vrp mySolution.sol -seed 1 -t 30
    68	```
    69	
    70	The following options are supported:
    71	```
    72	Call with: ./hgs instancePath solPath [-it nbIter] [-t myCPUtime] [-bks bksPath] [-seed mySeed] [-veh nbVehicles] [-log verbose]
    73	[-it <int>] sets a maximum number of iterations without improvement. Defaults to 20,000                                     
    74	[-t <double>] sets a time limit in seconds. If this parameter is set, the code will be run iteratively until the time limit           
    75	[-seed <int>] sets a fixed seed. Defaults to 0                                                                                    
    76	[-veh <int>] sets a prescribed fleet size. Otherwise a reasonable UB on the fleet size is calculated                      
    77	[-round <bool>] rounding the distance to the nearest integer or not. It can be 0 (not rounding) or 1 (rounding). Defaults to 1. 
    78	[-log <bool>] sets the verbose level of the algorithm log. It can be 0 or 1. Defaults to 1.                                       
    79	
    80	Additional Arguments:
    81	[-nbIterTraces <int>] Number of iterations between traces display during HGS execution. Defaults to 500
    82	[-nbGranular <int>] Granular search parameter, limits the number of moves in the RI local search. Defaults to 20               
    83	[-mu <int>] Minimum population size. Defaults to 25                                                                            
    84	[-lambda <int>] Number of solutions created before reaching the maximum population size (i.e., generation size). Defaults to 40
    85	[-nbElite <int>] Number of elite individuals. Defaults to 5                                                                    
    86	[-nbClose <int>] Number of closest solutions/individuals considered when calculating diversity contribution. Defaults to 4     
    87	[-nbIterPenaltyManagement <int>] Number of iterations between penalty updates. Defaults to 100
    88	[-targetFeasible <double>] target ratio of feasible individuals between penalty updates. Defaults to 0.2
    89	[-penaltyIncrease <double>] penalty increase if insufficient feasible individuals between penalty updates. Defaults to 1.2
    90	[-penaltyDecrease <double>] penalty decrease if sufficient feasible individuals between penalty updates. Defaults to 0.85
    91	```
    92	
    93	There exist different conventions regarding distance calculations in the academic literature.
    94	The default code behavior is to apply integer rounding, as it should be done on the X instances of Uchoa et al. (2017).
    95	To change this behavior (e.g., when testing on the CMT or Golden instances), give a flag `-round 0`, when you run the executable.
    96	
    97	The progress of the algorithm in the standard output will be displayed as:
    98	
    99	``
   100	It [N1] [N2] | T(s) [T] | Feas [NF] [BestF] [AvgF] | Inf [NI] [BestI] [AvgI] | Div [DivF] [DivI] | Feas [FeasC] [FeasD] | Pen [PenC] [PenD]
   101	``
   102	```
   103	[N1] and [N2]: Total number of iterations and iterations without improvement
   104	[T]: CPU time spent until now
   105	[NF] and [NI]: Number of feasible and infeasible solutions in the subpopulations 
   106	[BestF] and [BestI]: Value of the best feasible and infeasible solution in the subpopulations 
   107	[AvgF] and [AvgI]: Average value of the solutions in the feasible and infeasible subpopulations 
   108	[DivF] and [DivI]: Diversity of the feasible and infeasible subpopulations
   109	[FC] and [FD]: Percentage of naturally feasible solutions in relation to the capacity and duration constraints
   110	[PC] and [PD]: Current penalty level per unit of excess capacity and duration
   111	```
   112	
   113	## Code structure
   114	
   115	The main classes containing the logic of the algorithm are the following:
   116	* **Params**: Stores the main data structures for the method
   117	* **Individual**: Represents an individual solution in the genetic algorithm, also provides I/O functions to read and write individual solutions in CVRPLib format.
   118	* **Population**: Stores the solutions of the genetic algorithm into two different groups according to their feasibility. Also includes the functions in charge of diversity management.
   119	* **Genetic**: Contains the main procedures of the genetic algorithm as well as the crossover
   120	* **LocalSearch**: Includes the local search functions, including the SWAP* neighborhood
   121	* **Split**: Algorithms designed to decode solutions represented as giant tours into complete CVRP solutions
   122	* **CircleSector**: Small code used to represent and manage arc sectors (to efficiently restrict the SWAP* neighborhood)
   123	
   124	In addition, additional classes have been created to facilitate interfacing:
   125	* **AlgorithmParameters**: Stores the parameters of the algorithm
   126	* **CVRPLIB** Contains the instance data and functions designed to read input data as text files according to the CVRPLIB conventions
   127	* **commandline**: Reads the line of command
   128	* **main**: Main code to start the algorithm
   129	* **C_Interface**: Provides a C interface for the method
   130	
   131	## Compiling the shared library
   132	
   133	You can also build a shared library to call the HGS-CVRP algorithm from your code.
   134	
   135	```console
   136	mkdir build
   137	cd build
   138	cmake .. -DCMAKE_BUILD_TYPE=Release -G "Unix Makefiles"
   139	make lib
   140	```
   141	This will generate the library file, `libhgscvrp.so` (Linux), `libhgscvrp.dylib` (macOS), or `hgscvrp.dll` (Windows),
   142	in the `build` directory.
   143	
   144	To test calling the shared library from a C code:
   145	```console
   146	make lib_test_c
   147	ctest -R lib --verbose
   148	```
   149	
   150	## Contributing
   151	
   152	Thank you very much for your interest in this code.
   153	This code is still actively maintained and evolving. Pull requests and contributions seeking to improve the code in terms of readability, usability, and performance are welcome. Development is conducted in the `dev` branch. I recommend to contact me beforehand at <thibaut.vidal@polymtl.ca> before any major rework.
   154	
   155	As a general guideline, the goal of this code is to stay **simple**, **stand-alone**, and **specialized** to the CVRP. 
   156	Contributions that aim to extend this approach to different variants of the vehicle routing problem should usually remain in a separate repository.
   157	Similarly, additional libraries or significant increases of conceptual complexity will be avoided. Indeed, when developing (meta-)heuristics, it seems always possible to do a bit better at the cost of extra conceptual complexity. The overarching goal of this code is to find a good trade-off between algorithm simplicity and performance.
   158	
   159	There are two main types of contributions:
   160	* Changes that do not impact the sequence of solutions found by the HGS algorithm when running `ctest` and testing other instances with a fixed seed.
   161	This is visible by comparing the average solution value in the population and diversity through a test run. Such contributions include refactoring, simplification, and code optimization. Pull requests of this type are likely to be integrated more quickly.
   162	* Changes that impact the sequence of solutions found by the algorithm.
   163	In this case, I recommend to contact me beforehand with (i) a detailed description of the changes, (ii) detailed results on 10 runs of the algorithm for each of the 100 instances of Uchoa et al. (2017) before and after the changes, using the same termination criterion as used in [2](https://arxiv.org/abs/2012.10384).
   164	
   165	## License
   166	
   167	[![License](http://img.shields.io/:license-mit-blue.svg?style=flat-square)](http://badges.mit-license.org)
   168	
   169	- **[MIT license](http://opensource.org/licenses/mit-license.php)**
   170	- Copyright(c) 2020 Thibaut Vidal

````

</details>
## Assistant

<details>
<summary>思考过程</summary>

I'm going to examine the C++ source files to get a better understanding of how the implementation works.

</details>

现在让我阅读所有 C++ 源文件来理解实现细节。

<details>
<summary>Tool Call: Read file `Program/Params.h`</summary>

*Status: Completed*

```HGS-CVRP/Program/Params.h
     1	/*MIT License
     2	
     3	Copyright(c) 2020 Thibaut Vidal
     4	
     5	Permission is hereby granted, free of charge, to any person obtaining a copy
     6	of this software and associated documentation files(the "Software"), to deal
     7	in the Software without restriction, including without limitation the rights
     8	to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
     9	copies of the Software, and to permit persons to whom the Software is
    10	furnished to do so, subject to the following conditions :
    11	
    12	The above copyright notice and this permission notice shall be included in all
    13	copies or substantial portions of the Software.
    14	
    15	THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    16	IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    17	FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE
    18	AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    19	LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    20	OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    21	SOFTWARE.*/
    22	
    23	#ifndef PARAMS_H
    24	#define PARAMS_H
    25	
    26	#include "CircleSector.h"
    27	#include "AlgorithmParameters.h"
    28	#include <string>
    29	#include <vector>
    30	#include <list>
    31	#include <set>
    32	#include <fstream>
    33	#include <iostream>
    34	#include <sstream>
    35	#include <cmath>
    36	#include <time.h>
    37	#include <climits>
    38	#include <algorithm>
    39	#include <unordered_set>
    40	#include <random>
    41	#define MY_EPSILON 0.00001 // Precision parameter, used to avoid numerical instabilities
    42	#define PI 3.14159265359
    43	
    44	struct Client
    45	{
    46		double coordX;			// Coordinate X
    47		double coordY;			// Coordinate Y
    48		double serviceDuration; // Service duration
    49		double demand;			// Demand
    50		int polarAngle;			// Polar angle of the client around the depot, measured in degrees and truncated for convenience
    51	};
    52	
    53	class Params
    54	{
    55	public:
    56	
    57		/* PARAMETERS OF THE GENETIC ALGORITHM */
    58		bool verbose;                       // Controls verbose level through the iterations
    59		AlgorithmParameters ap;	            // Main parameters of the HGS algorithm
    60	
    61		/* ADAPTIVE PENALTY COEFFICIENTS */
    62		double penaltyCapacity;				// Penalty for one unit of capacity excess (adapted through the search)
    63		double penaltyDuration;				// Penalty for one unit of duration excess (adapted through the search)
    64	
    65		/* START TIME OF THE ALGORITHM */
    66		clock_t startTime;                  // Start time of the optimization (set when Params is constructed)
    67	
    68		/* RANDOM NUMBER GENERATOR */       
    69		std::minstd_rand ran;               // Using the fastest and simplest LCG. The quality of random numbers is not critical for the LS, but speed is
    70	
    71		/* DATA OF THE PROBLEM INSTANCE */
    72		bool isDurationConstraint ;								// Indicates if the problem includes duration constraints
    73		int nbClients ;											// Number of clients (excluding the depot)
    74		int nbVehicles ;										// Number of vehicles
    75		double durationLimit;									// Route duration limit
    76		double vehicleCapacity;									// Capacity limit
    77		double totalDemand ;									// Total demand required by the clients
    78		double maxDemand;										// Maximum demand of a client
    79		double maxDist;											// Maximum distance between two clients
    80		std::vector< Client > cli ;								// Vector containing information on each client
    81		const std::vector< std::vector< double > >& timeCost;	// Distance matrix
    82		std::vector< std::vector< int > > correlatedVertices;	// Neighborhood restrictions: For each client, list of nearby customers
    83		bool areCoordinatesProvided;                            // Check if valid coordinates are provided
    84	
    85		// Initialization from a given data set
    86		Params(const std::vector<double>& x_coords,
    87			const std::vector<double>& y_coords,
    88			const std::vector<std::vector<double>>& dist_mtx,
    89			const std::vector<double>& service_time,
    90			const std::vector<double>& demands,
    91			double vehicleCapacity,
    92			double durationLimit,
    93			int nbVeh,
    94			bool isDurationConstraint,
    95			bool verbose,
    96			const AlgorithmParameters& ap);
    97	};
    98	#endif
    99	

```

</details>
<details>
<summary>Tool Call: Read file `Program/Params.cpp`</summary>

*Status: Completed*

```HGS-CVRP/Program/Params.cpp
     1	#include "Params.h"
     2	
     3	// The universal constructor for both executable and shared library
     4	// When the executable is run from the commandline,
     5	// it will first generate an CVRPLIB instance from .vrp file, then supply necessary information.
     6	Params::Params(
     7		const std::vector<double>& x_coords,
     8		const std::vector<double>& y_coords,
     9		const std::vector<std::vector<double>>& dist_mtx,
    10		const std::vector<double>& service_time,
    11		const std::vector<double>& demands,
    12		double vehicleCapacity,
    13		double durationLimit,
    14		int nbVeh,
    15		bool isDurationConstraint,
    16		bool verbose,
    17		const AlgorithmParameters& ap
    18	)
    19		: ap(ap), isDurationConstraint(isDurationConstraint), nbVehicles(nbVeh), durationLimit(durationLimit),
    20		  vehicleCapacity(vehicleCapacity), timeCost(dist_mtx), verbose(verbose)
    21	{
    22		// This marks the starting time of the algorithm
    23		startTime = clock();
    24	
    25		nbClients = (int)demands.size() - 1; // Need to substract the depot from the number of nodes
    26		totalDemand = 0.;
    27		maxDemand = 0.;
    28	
    29		// Initialize RNG
    30		ran.seed(ap.seed);
    31	
    32		// check if valid coordinates are provided
    33		areCoordinatesProvided = (demands.size() == x_coords.size()) && (demands.size() == y_coords.size());
    34	
    35		cli = std::vector<Client>(nbClients + 1);
    36		for (int i = 0; i <= nbClients; i++)
    37		{
    38			// If useSwapStar==false, x_coords and y_coords may be empty.
    39			if (ap.useSwapStar == 1 && areCoordinatesProvided)
    40			{
    41				cli[i].coordX = x_coords[i];
    42				cli[i].coordY = y_coords[i];
    43				cli[i].polarAngle = CircleSector::positive_mod(
    44					32768. * atan2(cli[i].coordY - cli[0].coordY, cli[i].coordX - cli[0].coordX) / PI);
    45			}
    46			else
    47			{
    48				cli[i].coordX = 0.0;
    49				cli[i].coordY = 0.0;
    50				cli[i].polarAngle = 0.0;
    51			}
    52	
    53			cli[i].serviceDuration = service_time[i];
    54			cli[i].demand = demands[i];
    55			if (cli[i].demand > maxDemand) maxDemand = cli[i].demand;
    56			totalDemand += cli[i].demand;
    57		}
    58	
    59		if (verbose && ap.useSwapStar == 1 && !areCoordinatesProvided)
    60			std::cout << "----- NO COORDINATES HAVE BEEN PROVIDED, SWAP* NEIGHBORHOOD WILL BE DEACTIVATED BY DEFAULT" << std::endl;
    61	
    62		// Default initialization if the number of vehicles has not been provided by the user
    63		if (nbVehicles == INT_MAX)
    64		{
    65			nbVehicles = (int)std::ceil(1.3*totalDemand/vehicleCapacity) + 3;  // Safety margin: 30% + 3 more vehicles than the trivial bin packing LB
    66			if (verbose) 
    67				std::cout << "----- FLEET SIZE WAS NOT SPECIFIED: DEFAULT INITIALIZATION TO " << nbVehicles << " VEHICLES" << std::endl;
    68		}
    69		else
    70		{
    71			if (verbose)
    72				std::cout << "----- FLEET SIZE SPECIFIED: SET TO " << nbVehicles << " VEHICLES" << std::endl;
    73		}
    74	
    75		// Calculation of the maximum distance
    76		maxDist = 0.;
    77		for (int i = 0; i <= nbClients; i++)
    78			for (int j = 0; j <= nbClients; j++)
    79				if (timeCost[i][j] > maxDist) maxDist = timeCost[i][j];
    80	
    81		// Calculation of the correlated vertices for each customer (for the granular restriction)
    82		correlatedVertices = std::vector<std::vector<int> >(nbClients + 1);
    83		std::vector<std::set<int> > setCorrelatedVertices = std::vector<std::set<int> >(nbClients + 1);
    84		std::vector<std::pair<double, int> > orderProximity;
    85		for (int i = 1; i <= nbClients; i++)
    86		{
    87			orderProximity.clear();
    88			for (int j = 1; j <= nbClients; j++)
    89				if (i != j) orderProximity.emplace_back(timeCost[i][j], j);
    90			std::sort(orderProximity.begin(), orderProximity.end());
    91	
    92			for (int j = 0; j < std::min<int>(ap.nbGranular, nbClients - 1); j++)
    93			{
    94				// If i is correlated with j, then j should be correlated with i
    95				setCorrelatedVertices[i].insert(orderProximity[j].second);
    96				setCorrelatedVertices[orderProximity[j].second].insert(i);
    97			}
    98		}
    99	
   100		// Filling the vector of correlated vertices
   101		for (int i = 1; i <= nbClients; i++)
   102			for (int x : setCorrelatedVertices[i])
   103				correlatedVertices[i].push_back(x);
   104	
   105		// Safeguards to avoid possible numerical instability in case of instances containing arbitrarily small or large numerical values
   106		if (maxDist < 0.1 || maxDist > 100000)
   107			throw std::string(
   108				"The distances are of very small or large scale. This could impact numerical stability. Please rescale the dataset and run again.");
   109		if (maxDemand < 0.1 || maxDemand > 100000)
   110			throw std::string(
   111				"The demand quantities are of very small or large scale. This could impact numerical stability. Please rescale the dataset and run again.");
   112		if (nbVehicles < std::ceil(totalDemand / vehicleCapacity))
   113			throw std::string("Fleet size is insufficient to service the considered clients.");
   114	
   115		// A reasonable scale for the initial values of the penalties
   116		penaltyDuration = 1;
   117		penaltyCapacity = std::max<double>(0.1, std::min<double>(1000., maxDist / maxDemand));
   118	
   119		if (verbose)
   120			std::cout << "----- INSTANCE SUCCESSFULLY LOADED WITH " << nbClients << " CLIENTS AND " << nbVehicles << " VEHICLES" << std::endl;
   121	}
   122	
   123	

```

</details>
<details>
<summary>Tool Call: Read file `Program/AlgorithmParameters.h`</summary>

*Status: Completed*

```HGS-CVRP/Program/AlgorithmParameters.h
     1	//
     2	// Created by chkwon on 3/23/22.
     3	//
     4	
     5	// This header file must be readable in C.
     6	
     7	#ifndef ALGORITHMPARAMETERS_H
     8	#define ALGORITHMPARAMETERS_H
     9	
    10	struct AlgorithmParameters {
    11		int nbGranular;			// Granular search parameter, limits the number of moves in the RI local search
    12		int mu;					// Minimum population size
    13		int lambda;				// Number of solutions created before reaching the maximum population size (i.e., generation size)
    14		int nbElite;			// Number of elite individuals
    15		int nbClose;			// Number of closest solutions/individuals considered when calculating diversity contribution
    16	
    17		int nbIterPenaltyManagement;  // Number of iterations between penalty updates
    18		double targetFeasible;	      // Reference proportion for the number of feasible individuals, used for the adaptation of the penalty parameters
    19		double penaltyDecrease;	      // Multiplier used to decrease penalty parameters if there are sufficient feasible individuals
    20		double penaltyIncrease;	      // Multiplier used to increase penalty parameters if there are insufficient feasible individuals
    21	
    22		int seed;				// Random seed. Default value: 0
    23		int nbIter;				// Nb iterations without improvement until termination (or restart if a time limit is specified). Default value: 20,000 iterations
    24		int nbIterTraces;       // Number of iterations between traces display during HGS execution
    25		double timeLimit;		// CPU time limit until termination in seconds. Default value: 0 (i.e., inactive)
    26		int useSwapStar;		// Use SWAP* local search or not. Default value: 1. Only available when coordinates are provided.
    27	};
    28	
    29	#ifdef __cplusplus
    30	extern "C"
    31	#endif
    32	struct AlgorithmParameters default_algorithm_parameters();
    33	
    34	#ifdef __cplusplus
    35	void print_algorithm_parameters(const AlgorithmParameters & ap);
    36	#endif
    37	
    38	#endif //ALGORITHMPARAMETERS_H

```

</details>
<details>
<summary>Tool Call: Read file `Program/AlgorithmParameters.cpp`</summary>

*Status: Completed*

```HGS-CVRP/Program/AlgorithmParameters.cpp
     1	//
     2	// Created by chkwon on 3/23/22.
     3	//
     4	
     5	#include "AlgorithmParameters.h"
     6	#include <iostream>
     7	
     8	extern "C"
     9	struct AlgorithmParameters default_algorithm_parameters() {
    10		struct AlgorithmParameters ap{};
    11	
    12		ap.nbGranular = 20;
    13		ap.mu = 25;
    14		ap.lambda = 40;
    15		ap.nbElite = 4;
    16		ap.nbClose = 5;
    17	
    18		ap.nbIterPenaltyManagement = 100;
    19		ap.targetFeasible = 0.2;
    20		ap.penaltyDecrease = 0.85;
    21		ap.penaltyIncrease = 1.2;
    22	
    23		ap.seed = 0;
    24		ap.nbIter = 20000;
    25		ap.nbIterTraces = 500;
    26		ap.timeLimit = 0;
    27		ap.useSwapStar = 1;
    28	
    29		return ap;
    30	}
    31	
    32	void print_algorithm_parameters(const AlgorithmParameters & ap)
    33	{
    34		std::cout << "=========== Algorithm Parameters =================" << std::endl;
    35		std::cout << "---- nbGranular              is set to " << ap.nbGranular << std::endl;
    36		std::cout << "---- mu                      is set to " << ap.mu << std::endl;
    37		std::cout << "---- lambda                  is set to " << ap.lambda << std::endl;
    38		std::cout << "---- nbElite                 is set to " << ap.nbElite << std::endl;
    39		std::cout << "---- nbClose                 is set to " << ap.nbClose << std::endl;
    40		std::cout << "---- nbIterPenaltyManagement is set to " << ap.nbIterPenaltyManagement << std::endl;
    41		std::cout << "---- targetFeasible          is set to " << ap.targetFeasible << std::endl;
    42		std::cout << "---- penaltyDecrease         is set to " << ap.penaltyDecrease << std::endl;
    43		std::cout << "---- penaltyIncrease         is set to " << ap.penaltyIncrease << std::endl;
    44		std::cout << "---- seed                    is set to " << ap.seed << std::endl;
    45		std::cout << "---- nbIter                  is set to " << ap.nbIter << std::endl;
    46		std::cout << "---- nbIterTraces            is set to " << ap.nbIterTraces << std::endl;
    47		std::cout << "---- timeLimit               is set to " << ap.timeLimit << std::endl;
    48		std::cout << "---- useSwapStar             is set to " << ap.useSwapStar << std::endl;
    49		std::cout << "==================================================" << std::endl;
    50	}

```

</details>
<details>
<summary>Tool Call: Read file `Program/CircleSector.h`</summary>

*Status: Completed*

```HGS-CVRP/Program/CircleSector.h
     1	#ifndef CIRCLESECTOR_H
     2	#define CIRCLESECTOR_H
     3	
     4	// Simple data structure to represent circle sectors
     5	// Angles are measured in [0,65535] instead of [0,359], in such a way that modulo operations are much faster (since 2^16 = 65536)
     6	// Credit to Fabian Giesen at "https://web.archive.org/web/20200912191950/https://fgiesen.wordpress.com/2015/09/24/intervals-in-modular-arithmetic/" for useful implementation tips regarding interval overlaps in modular arithmetics 
     7	struct CircleSector
     8	{
     9		int start;
    10		int end;
    11	
    12		// Positive modulo 65536
    13		static int positive_mod(int i)
    14		{
    15			// 1) Using the formula positive_mod(n,x) = (n % x + x) % x
    16			// 2) Moreover, remark that "n % 65536" should be automatically compiled in an optimized form as "n & 0xffff" for faster calculations
    17			return (i % 65536 + 65536) % 65536;
    18		}
    19	
    20		// Initialize a circle sector from a single point
    21		void initialize(int point)
    22		{
    23			start = point;
    24			end = point;
    25		}
    26	
    27		// Tests if a point is enclosed in the circle sector
    28		bool isEnclosed(int point)
    29		{
    30			return (positive_mod(point - start) <= positive_mod(end - start));
    31		}
    32	
    33		// Tests overlap of two circle sectors
    34		static bool overlap(const CircleSector & sector1, const CircleSector & sector2)
    35		{
    36			return ((positive_mod(sector2.start - sector1.start) <= positive_mod(sector1.end - sector1.start))
    37				|| (positive_mod(sector1.start - sector2.start) <= positive_mod(sector2.end - sector2.start)));
    38		}
    39	
    40		// Extends the circle sector to include an additional point 
    41		// Done in a "greedy" way, such that the resulting circle sector is the smallest
    42		void extend(int point)
    43		{
    44			if (!isEnclosed(point))
    45			{
    46				if (positive_mod(point - end) <= positive_mod(start - point))
    47					end = point;
    48				else
    49					start = point;
    50			}
    51		}
    52	};
    53	
    54	#endif

```

</details>
<details>
<summary>Tool Call: Read file `Program/Individual.h`</summary>

*Status: Completed*

```HGS-CVRP/Program/Individual.h
     1	/*MIT License
     2	
     3	Copyright(c) 2020 Thibaut Vidal
     4	
     5	Permission is hereby granted, free of charge, to any person obtaining a copy
     6	of this software and associated documentation files(the "Software"), to deal
     7	in the Software without restriction, including without limitation the rights
     8	to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
     9	copies of the Software, and to permit persons to whom the Software is
    10	furnished to do so, subject to the following conditions :
    11	
    12	The above copyright notice and this permission notice shall be included in all
    13	copies or substantial portions of the Software.
    14	
    15	THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    16	IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    17	FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE
    18	AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    19	LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    20	OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    21	SOFTWARE.*/
    22	
    23	#ifndef INDIVIDUAL_H
    24	#define INDIVIDUAL_H
    25	
    26	#include "Params.h"
    27	
    28	struct EvalIndiv
    29	{
    30		double penalizedCost = 0.;		// Penalized cost of the solution
    31		int nbRoutes = 0;				// Number of routes
    32		double distance = 0.;			// Total distance
    33		double capacityExcess = 0.;		// Sum of excess load in all routes
    34		double durationExcess = 0.;		// Sum of excess duration in all routes
    35		bool isFeasible = false;		// Feasibility status of the individual
    36	};
    37	
    38	class Individual
    39	{
    40	public:
    41	
    42	  EvalIndiv eval;															// Solution cost parameters
    43	  std::vector < int > chromT ;												// Giant tour representing the individual
    44	  std::vector < std::vector <int> > chromR ;								// For each vehicle, the associated sequence of deliveries (complete solution)
    45	  std::vector < int > successors ;											// For each node, the successor in the solution (can be the depot 0)
    46	  std::vector < int > predecessors ;										// For each node, the predecessor in the solution (can be the depot 0)
    47	  std::multiset < std::pair < double, Individual* > > indivsPerProximity ;	// The other individuals in the population, ordered by increasing proximity (the set container follows a natural ordering based on the first value of the pair)
    48	  double biasedFitness;														// Biased fitness of the solution
    49	
    50	  // Measuring cost and feasibility of an Individual from the information of chromR (needs chromR filled and access to Params)
    51	  void evaluateCompleteCost(const Params & params);
    52	
    53	  // Constructor of a random individual containing only a giant tour with a shuffled visit order
    54	  Individual(Params & params);
    55	
    56	  // Constructor of an individual from a file in CVRPLib solution format as produced by the algorithm (useful if a user wishes to input an initial solution)
    57	  Individual(Params & params, std::string fileName);
    58	};
    59	#endif

```

</details>
<details>
<summary>Tool Call: Read file `Program/Individual.cpp`</summary>

*Status: Completed*

```HGS-CVRP/Program/Individual.cpp
     1	#include "Individual.h" 
     2	
     3	void Individual::evaluateCompleteCost(const Params & params)
     4	{
     5		eval = EvalIndiv();
     6		for (int r = 0; r < params.nbVehicles; r++)
     7		{
     8			if (!chromR[r].empty())
     9			{
    10				double distance = params.timeCost[0][chromR[r][0]];
    11				double load = params.cli[chromR[r][0]].demand;
    12				double service = params.cli[chromR[r][0]].serviceDuration;
    13				predecessors[chromR[r][0]] = 0;
    14				for (int i = 1; i < (int)chromR[r].size(); i++)
    15				{
    16					distance += params.timeCost[chromR[r][i-1]][chromR[r][i]];
    17					load += params.cli[chromR[r][i]].demand;
    18					service += params.cli[chromR[r][i]].serviceDuration;
    19					predecessors[chromR[r][i]] = chromR[r][i-1];
    20					successors[chromR[r][i-1]] = chromR[r][i];
    21				}
    22				successors[chromR[r][chromR[r].size()-1]] = 0;
    23				distance += params.timeCost[chromR[r][chromR[r].size()-1]][0];
    24				eval.distance += distance;
    25				eval.nbRoutes++;
    26				if (load > params.vehicleCapacity) eval.capacityExcess += load - params.vehicleCapacity;
    27				if (distance + service > params.durationLimit) eval.durationExcess += distance + service - params.durationLimit;
    28			}
    29		}
    30	
    31		eval.penalizedCost = eval.distance + eval.capacityExcess*params.penaltyCapacity + eval.durationExcess*params.penaltyDuration;
    32		eval.isFeasible = (eval.capacityExcess < MY_EPSILON && eval.durationExcess < MY_EPSILON);
    33	}
    34	
    35	Individual::Individual(Params & params)
    36	{
    37		successors = std::vector <int>(params.nbClients + 1);
    38		predecessors = std::vector <int>(params.nbClients + 1);
    39		chromR = std::vector < std::vector <int> >(params.nbVehicles);
    40		chromT = std::vector <int>(params.nbClients);
    41		for (int i = 0; i < params.nbClients; i++) chromT[i] = i + 1;
    42		std::shuffle(chromT.begin(), chromT.end(), params.ran);
    43		eval.penalizedCost = 1.e30;	
    44	}
    45	
    46	Individual::Individual(Params & params, std::string fileName) : Individual(params)
    47	{
    48		double readCost;
    49		chromT.clear();
    50		std::ifstream inputFile(fileName);
    51		if (inputFile.is_open())
    52		{
    53			std::string inputString;
    54			inputFile >> inputString;
    55			// Loops in the input file as long as the first line keyword is "Route"
    56			for (int r = 0; inputString == "Route"; r++)
    57			{
    58				inputFile >> inputString;
    59				getline(inputFile, inputString);
    60				std::stringstream ss(inputString);
    61				int inputCustomer;
    62				while (ss >> inputCustomer) // Loops as long as there is an integer to read in this route
    63				{
    64					chromT.push_back(inputCustomer);
    65					chromR[r].push_back(inputCustomer);
    66				}
    67				inputFile >> inputString;
    68			}
    69			if (inputString == "Cost") inputFile >> readCost;
    70			else throw std::string("Unexpected token in input solution");
    71	
    72			// Some safety checks and printouts
    73			evaluateCompleteCost(params);
    74			if ((int)chromT.size() != params.nbClients) throw std::string("Input solution does not contain the correct number of clients");
    75			if (!eval.isFeasible) throw std::string("Input solution is infeasible");
    76			if (eval.penalizedCost != readCost)throw std::string("Input solution has a different cost than announced in the file");
    77			if (params.verbose) std::cout << "----- INPUT SOLUTION HAS BEEN SUCCESSFULLY READ WITH COST " << eval.penalizedCost << std::endl;
    78		}
    79		else 
    80			throw std::string("Impossible to open solution file provided in input in : " + fileName);
    81	}

```

</details>
<details>
<summary>Tool Call: Read file `Program/Population.h`</summary>

*Status: Completed*

```HGS-CVRP/Program/Population.h
     1	/*MIT License
     2	
     3	Copyright(c) 2020 Thibaut Vidal
     4	
     5	Permission is hereby granted, free of charge, to any person obtaining a copy
     6	of this software and associated documentation files(the "Software"), to deal
     7	in the Software without restriction, including without limitation the rights
     8	to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
     9	copies of the Software, and to permit persons to whom the Software is
    10	furnished to do so, subject to the following conditions :
    11	
    12	The above copyright notice and this permission notice shall be included in all
    13	copies or substantial portions of the Software.
    14	
    15	THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    16	IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    17	FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE
    18	AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    19	LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    20	OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    21	SOFTWARE.*/
    22	
    23	#ifndef POPULATION_H
    24	#define POPULATION_H
    25	
    26	#include "Individual.h"
    27	#include "LocalSearch.h"
    28	#include "Split.h"
    29	
    30	typedef std::vector <Individual*> SubPopulation ;
    31	
    32	class Population
    33	{
    34	   private:
    35	
    36	   Params & params ;							// Problem parameters
    37	   Split & split;								// Split algorithm
    38	   LocalSearch & localSearch;					// Local search structure
    39	   SubPopulation feasibleSubpop;			    // Feasible subpopulation, kept ordered by increasing penalized cost
    40	   SubPopulation infeasibleSubpop;		        // Infeasible subpopulation, kept ordered by increasing penalized cost
    41	   std::list <bool> listFeasibilityLoad ;		// Load feasibility of recent individuals generated by LS
    42	   std::list <bool> listFeasibilityDuration ;	// Duration feasibility of recent individuals generated by LS
    43	   std::vector<std::pair<clock_t, double>> searchProgress; // Keeps tracks of the time stamps of successive best solutions
    44	   Individual bestSolutionRestart;              // Best solution found during the current restart of the algorithm
    45	   Individual bestSolutionOverall;              // Best solution found during the complete execution of the algorithm
    46	
    47	   // Evaluates the biased fitness of all individuals in the population
    48	   void updateBiasedFitnesses(SubPopulation & pop);
    49	
    50	   // Removes the worst individual in terms of biased fitness
    51	   void removeWorstBiasedFitness(SubPopulation & subpop);
    52	
    53	   public:
    54	
    55	   // Creates an initial population of individuals
    56	   void generatePopulation();
    57	
    58	   // Add an individual in the population (survivor selection is automatically triggered whenever the population reaches its maximum size)
    59	   // Returns TRUE if a new best solution of the run has been found
    60	   bool addIndividual (const Individual & indiv, bool updateFeasible);
    61	
    62	   // Cleans all solutions and generates a new initial population (only used when running HGS until a time limit, in which case the algorithm restarts until the time limit is reached)
    63	   void restart();
    64	
    65	   // Adaptation of the penalty parameters
    66	   void managePenalties();
    67	
    68	   // Select an individal by binary tournament in the union of the feasible and infeasible subpopulations.
    69	   const Individual & getBinaryTournament();
    70	
    71	   // Accesses the best feasible individual
    72	   const Individual * getBestFeasible();
    73	
    74	   // Accesses the best infeasible individual
    75	   const Individual * getBestInfeasible();
    76	
    77	   // Accesses the best found solution at all time
    78	   const Individual * getBestFound();
    79	
    80	   // Prints population state
    81	   void printState(int nbIter, int nbIterNoImprovement);
    82	
    83	   // Distance measure between two individuals, used for diversity calculations
    84	   double brokenPairsDistance(const Individual & indiv1, const Individual & indiv2);
    85	
    86	   // Returns the average broken pairs distance of this individual with the nbClosest individuals in the population
    87	   double averageBrokenPairsDistanceClosest(const Individual & indiv, int nbClosest);
    88	
    89	   // Returns the average diversity value among the 50% best individuals in the subpopulation
    90	   double getDiversity(const SubPopulation & pop);
    91	
    92	   // Returns the average solution value among the 50% best individuals in the subpopulation
    93	   double getAverageCost(const SubPopulation & pop);
    94	
    95	   // Exports in a file the history of solution improvements
    96	   void exportSearchProgress(std::string fileName, std::string instanceName);
    97	
    98	   // Exports an Individual in CVRPLib format
    99	   void exportCVRPLibFormat(const Individual & indiv, std::string fileName);
   100	
   101	   // Constructor
   102	   Population(Params & params, Split & split, LocalSearch & localSearch);
   103	
   104	   // Destructor
   105	   ~Population();
   106	};
   107	
   108	#endif

```

</details>
<details>
<summary>Tool Call: Read file `Program/Population.cpp`</summary>

*Status: Completed*

```HGS-CVRP/Program/Population.cpp
     1	#include "Population.h"
     2	
     3	void Population::generatePopulation()
     4	{
     5		if (params.verbose) std::cout << "----- BUILDING INITIAL POPULATION" << std::endl;
     6		for (int i = 0; i < 4*params.ap.mu && (i == 0 || params.ap.timeLimit == 0 || (double)(clock() - params.startTime) / (double)CLOCKS_PER_SEC < params.ap.timeLimit) ; i++)
     7		{
     8			Individual randomIndiv(params);
     9			split.generalSplit(randomIndiv, params.nbVehicles);
    10			localSearch.run(randomIndiv, params.penaltyCapacity, params.penaltyDuration);
    11			addIndividual(randomIndiv, true);
    12			if (!randomIndiv.eval.isFeasible && params.ran() % 2 == 0)  // Repair half of the solutions in case of infeasibility
    13			{
    14				localSearch.run(randomIndiv, params.penaltyCapacity*10., params.penaltyDuration*10.);
    15				if (randomIndiv.eval.isFeasible) addIndividual(randomIndiv, false);
    16			}
    17		}
    18	}
    19	
    20	bool Population::addIndividual(const Individual & indiv, bool updateFeasible)
    21	{
    22		if (updateFeasible)
    23		{
    24			listFeasibilityLoad.push_back(indiv.eval.capacityExcess < MY_EPSILON);
    25			listFeasibilityDuration.push_back(indiv.eval.durationExcess < MY_EPSILON);
    26			listFeasibilityLoad.pop_front();
    27			listFeasibilityDuration.pop_front();
    28		}
    29	
    30		// Find the adequate subpopulation in relation to the individual feasibility
    31		SubPopulation & subpop = (indiv.eval.isFeasible) ? feasibleSubpop : infeasibleSubpop;
    32	
    33		// Create a copy of the individual and updade the proximity structures calculating inter-individual distances
    34		Individual * myIndividual = new Individual(indiv);
    35		for (Individual * myIndividual2 : subpop)
    36		{
    37			double myDistance = brokenPairsDistance(*myIndividual,*myIndividual2);
    38			myIndividual2->indivsPerProximity.insert({ myDistance, myIndividual });
    39			myIndividual->indivsPerProximity.insert({ myDistance, myIndividual2 });
    40		}
    41	
    42		// Identify the correct location in the subpopulation and insert the individual
    43		int place = (int)subpop.size();
    44		while (place > 0 && subpop[place - 1]->eval.penalizedCost > indiv.eval.penalizedCost - MY_EPSILON) place--;
    45		subpop.emplace(subpop.begin() + place, myIndividual);
    46	
    47		// Trigger a survivor selection if the maximimum subpopulation size is exceeded
    48		if ((int)subpop.size() > params.ap.mu + params.ap.lambda)
    49			while ((int)subpop.size() > params.ap.mu)
    50				removeWorstBiasedFitness(subpop);
    51	
    52		// Track best solution
    53		if (indiv.eval.isFeasible && indiv.eval.penalizedCost < bestSolutionRestart.eval.penalizedCost - MY_EPSILON)
    54		{
    55			bestSolutionRestart = indiv; // Copy
    56			if (indiv.eval.penalizedCost < bestSolutionOverall.eval.penalizedCost - MY_EPSILON)
    57			{
    58				bestSolutionOverall = indiv;
    59				searchProgress.push_back({ clock() - params.startTime , bestSolutionOverall.eval.penalizedCost });
    60			}
    61			return true;
    62		}
    63		else
    64			return false;
    65	}
    66	
    67	void Population::updateBiasedFitnesses(SubPopulation & pop)
    68	{
    69		// Ranking the individuals based on their diversity contribution (decreasing order of distance)
    70		std::vector <std::pair <double, int> > ranking;
    71		for (int i = 0 ; i < (int)pop.size(); i++) 
    72			ranking.push_back({-averageBrokenPairsDistanceClosest(*pop[i],params.ap.nbClose),i});
    73		std::sort(ranking.begin(), ranking.end());
    74	
    75		// Updating the biased fitness values
    76		if (pop.size() == 1) 
    77			pop[0]->biasedFitness = 0;
    78		else
    79		{
    80			for (int i = 0; i < (int)pop.size(); i++)
    81			{
    82				double divRank = (double)i / (double)(pop.size() - 1); // Ranking from 0 to 1
    83				double fitRank = (double)ranking[i].second / (double)(pop.size() - 1);
    84				if ((int)pop.size() <= params.ap.nbElite) // Elite individuals cannot be smaller than population size
    85					pop[ranking[i].second]->biasedFitness = fitRank;
    86				else 
    87					pop[ranking[i].second]->biasedFitness = fitRank + (1.0 - (double)params.ap.nbElite / (double)pop.size()) * divRank;
    88			}
    89		}
    90	}
    91	
    92	void Population::removeWorstBiasedFitness(SubPopulation & pop)
    93	{
    94		updateBiasedFitnesses(pop);
    95		if (pop.size() <= 1) throw std::string("Eliminating the best individual: this should not occur in HGS");
    96	
    97		Individual * worstIndividual = NULL;
    98		int worstIndividualPosition = -1;
    99		bool isWorstIndividualClone = false;
   100		double worstIndividualBiasedFitness = -1.e30;
   101		for (int i = 1; i < (int)pop.size(); i++)
   102		{
   103			bool isClone = (averageBrokenPairsDistanceClosest(*pop[i],1) < MY_EPSILON); // A distance equal to 0 indicates that a clone exists
   104			if ((isClone && !isWorstIndividualClone) || (isClone == isWorstIndividualClone && pop[i]->biasedFitness > worstIndividualBiasedFitness))
   105			{
   106				worstIndividualBiasedFitness = pop[i]->biasedFitness;
   107				isWorstIndividualClone = isClone;
   108				worstIndividualPosition = i;
   109				worstIndividual = pop[i];
   110			}
   111		}
   112	
   113		// Removing the individual from the population and freeing memory
   114		pop.erase(pop.begin() + worstIndividualPosition); 
   115	
   116		// Cleaning its distances from the other individuals in the population
   117		for (Individual * indiv2 : pop)
   118		{
   119			auto it = indiv2->indivsPerProximity.begin();
   120			while (it->second != worstIndividual) ++it;
   121			indiv2->indivsPerProximity.erase(it);
   122		}
   123	
   124		// Freeing memory
   125		delete worstIndividual; 
   126	}
   127	
   128	void Population::restart()
   129	{
   130		if (params.verbose) std::cout << "----- RESET: CREATING A NEW POPULATION -----" << std::endl;
   131		for (Individual * indiv : feasibleSubpop) delete indiv ;
   132		for (Individual * indiv : infeasibleSubpop) delete indiv;
   133		feasibleSubpop.clear();
   134		infeasibleSubpop.clear();
   135		bestSolutionRestart = Individual(params);
   136		generatePopulation();
   137	}
   138	
   139	void Population::managePenalties()
   140	{
   141		// Setting some bounds [0.1,100000] to the penalty values for safety
   142		double fractionFeasibleLoad = (double)std::count(listFeasibilityLoad.begin(), listFeasibilityLoad.end(), true) / (double)listFeasibilityLoad.size();
   143		if (fractionFeasibleLoad < params.ap.targetFeasible - 0.05 && params.penaltyCapacity < 100000.)
   144			params.penaltyCapacity = std::min<double>(params.penaltyCapacity * params.ap.penaltyIncrease, 100000.);
   145		else if (fractionFeasibleLoad > params.ap.targetFeasible + 0.05 && params.penaltyCapacity > 0.1)
   146			params.penaltyCapacity = std::max<double>(params.penaltyCapacity * params.ap.penaltyDecrease, 0.1);
   147	
   148		// Setting some bounds [0.1,100000] to the penalty values for safety
   149		double fractionFeasibleDuration = (double)std::count(listFeasibilityDuration.begin(), listFeasibilityDuration.end(), true) / (double)listFeasibilityDuration.size();
   150		if (fractionFeasibleDuration < params.ap.targetFeasible - 0.05 && params.penaltyDuration < 100000.)
   151			params.penaltyDuration = std::min<double>(params.penaltyDuration * params.ap.penaltyIncrease, 100000.);
   152		else if (fractionFeasibleDuration > params.ap.targetFeasible + 0.05 && params.penaltyDuration > 0.1)
   153			params.penaltyDuration = std::max<double>(params.penaltyDuration * params.ap.penaltyDecrease, 0.1);
   154	
   155		// Update the evaluations
   156		for (int i = 0; i < (int)infeasibleSubpop.size(); i++)
   157			infeasibleSubpop[i]->eval.penalizedCost = infeasibleSubpop[i]->eval.distance
   158			+ params.penaltyCapacity * infeasibleSubpop[i]->eval.capacityExcess
   159			+ params.penaltyDuration * infeasibleSubpop[i]->eval.durationExcess;
   160	
   161		// If needed, reorder the individuals in the infeasible subpopulation since the penalty values have changed (simple bubble sort for the sake of simplicity)
   162		for (int i = 0; i < (int)infeasibleSubpop.size(); i++)
   163		{
   164			for (int j = 0; j < (int)infeasibleSubpop.size() - i - 1; j++)
   165			{
   166				if (infeasibleSubpop[j]->eval.penalizedCost > infeasibleSubpop[j + 1]->eval.penalizedCost + MY_EPSILON)
   167				{
   168					Individual * indiv = infeasibleSubpop[j];
   169					infeasibleSubpop[j] = infeasibleSubpop[j + 1];
   170					infeasibleSubpop[j + 1] = indiv;
   171				}
   172			}
   173		}
   174	}
   175	
   176	const Individual & Population::getBinaryTournament ()
   177	{
   178		// Picking two individuals with uniform distribution over the union of the feasible and infeasible subpopulations
   179		std::uniform_int_distribution<> distr(0, feasibleSubpop.size() + infeasibleSubpop.size() - 1);
   180		int place1 = distr(params.ran);
   181		int place2 = distr(params.ran);
   182		Individual * indiv1 = (place1 >= (int)feasibleSubpop.size()) ? infeasibleSubpop[place1 - feasibleSubpop.size()] : feasibleSubpop[place1];
   183		Individual * indiv2 = (place2 >= (int)feasibleSubpop.size()) ? infeasibleSubpop[place2 - feasibleSubpop.size()] : feasibleSubpop[place2];
   184		
   185		// Keeping the best of the two in terms of biased fitness
   186		updateBiasedFitnesses(feasibleSubpop);
   187		updateBiasedFitnesses(infeasibleSubpop);
   188		if (indiv1->biasedFitness < indiv2->biasedFitness) return *indiv1 ;
   189		else return *indiv2 ;		
   190	}
   191	
   192	const Individual * Population::getBestFeasible ()
   193	{
   194		if (!feasibleSubpop.empty()) return feasibleSubpop[0] ;
   195		else return NULL ;
   196	}
   197	
   198	const Individual * Population::getBestInfeasible ()
   199	{
   200		if (!infeasibleSubpop.empty()) return infeasibleSubpop[0] ;
   201		else return NULL ;
   202	}
   203	
   204	const Individual * Population::getBestFound()
   205	{
   206		if (bestSolutionOverall.eval.penalizedCost < 1.e29) return &bestSolutionOverall;
   207		else return NULL;
   208	}
   209	
   210	void Population::printState(int nbIter, int nbIterNoImprovement)
   211	{
   212		if (params.verbose)
   213		{
   214			std::printf("It %6d %6d | T(s) %.2f", nbIter, nbIterNoImprovement, (double)(clock()-params.startTime)/(double)CLOCKS_PER_SEC);
   215	
   216			if (getBestFeasible() != NULL) std::printf(" | Feas %zu %.2f %.2f", feasibleSubpop.size(), getBestFeasible()->eval.penalizedCost, getAverageCost(feasibleSubpop));
   217			else std::printf(" | NO-FEASIBLE");
   218	
   219			if (getBestInfeasible() != NULL) std::printf(" | Inf %zu %.2f %.2f", infeasibleSubpop.size(), getBestInfeasible()->eval.penalizedCost, getAverageCost(infeasibleSubpop));
   220			else std::printf(" | NO-INFEASIBLE");
   221	
   222			std::printf(" | Div %.2f %.2f", getDiversity(feasibleSubpop), getDiversity(infeasibleSubpop));
   223			std::printf(" | Feas %.2f %.2f", (double)std::count(listFeasibilityLoad.begin(), listFeasibilityLoad.end(), true) / (double)listFeasibilityLoad.size(), (double)std::count(listFeasibilityDuration.begin(), listFeasibilityDuration.end(), true) / (double)listFeasibilityDuration.size());
   224			std::printf(" | Pen %.2f %.2f", params.penaltyCapacity, params.penaltyDuration);
   225			std::cout << std::endl;
   226		}
   227	}
   228	
   229	double Population::brokenPairsDistance(const Individual & indiv1, const Individual & indiv2)
   230	{
   231		int differences = 0;
   232		for (int j = 1; j <= params.nbClients; j++)
   233		{
   234			if (indiv1.successors[j] != indiv2.successors[j] && indiv1.successors[j] != indiv2.predecessors[j]) differences++;
   235			if (indiv1.predecessors[j] == 0 && indiv2.predecessors[j] != 0 && indiv2.successors[j] != 0) differences++;
   236		}
   237		return (double)differences / (double)params.nbClients;
   238	}
   239	
   240	double Population::averageBrokenPairsDistanceClosest(const Individual & indiv, int nbClosest)
   241	{
   242		double result = 0.;
   243		int maxSize = std::min<int>(nbClosest, indiv.indivsPerProximity.size());
   244		auto it = indiv.indivsPerProximity.begin();
   245		for (int i = 0; i < maxSize; i++)
   246		{
   247			result += it->first;
   248			++it;
   249		}
   250		return result / (double)maxSize;
   251	}
   252	
   253	double Population::getDiversity(const SubPopulation & pop)
   254	{
   255		double average = 0.;
   256		int size = std::min<int>(params.ap.mu, pop.size()); // Only monitoring the "mu" better solutions to avoid too much noise in the measurements
   257		for (int i = 0; i < size; i++) average += averageBrokenPairsDistanceClosest(*pop[i],size);
   258		if (size > 0) return average / (double)size;
   259		else return -1.0;
   260	}
   261	
   262	double Population::getAverageCost(const SubPopulation & pop)
   263	{
   264		double average = 0.;
   265		int size = std::min<int>(params.ap.mu, pop.size()); // Only monitoring the "mu" better solutions to avoid too much noise in the measurements
   266		for (int i = 0; i < size; i++) average += pop[i]->eval.penalizedCost;
   267		if (size > 0) return average / (double)size;
   268		else return -1.0;
   269	}
   270	
   271	void Population::exportSearchProgress(std::string fileName, std::string instanceName)
   272	{
   273		std::ofstream myfile(fileName);
   274		for (std::pair<clock_t, double> state : searchProgress)
   275			myfile << instanceName << ";" << params.ap.seed << ";" << state.second << ";" << (double)state.first / (double)CLOCKS_PER_SEC << std::endl;
   276	}
   277	
   278	void Population::exportCVRPLibFormat(const Individual & indiv, std::string fileName)
   279	{
   280		std::ofstream myfile(fileName);
   281		if (myfile.is_open())
   282		{
   283			for (int k = 0; k < (int)indiv.chromR.size(); k++)
   284			{
   285				if (!indiv.chromR[k].empty())
   286				{
   287					myfile << "Route #" << k + 1 << ":"; // Route IDs start at 1 in the file format
   288					for (int i : indiv.chromR[k]) myfile << " " << i;
   289					myfile << std::endl;
   290				}
   291			}
   292			myfile << "Cost " << indiv.eval.penalizedCost << std::endl;
   293		}
   294		else std::cout << "----- IMPOSSIBLE TO OPEN: " << fileName << std::endl;
   295	}
   296	
   297	Population::Population(Params & params, Split & split, LocalSearch & localSearch) : params(params), split(split), localSearch(localSearch), bestSolutionRestart(params), bestSolutionOverall(params)
   298	{
   299		listFeasibilityLoad = std::list<bool>(params.ap.nbIterPenaltyManagement, true);
   300		listFeasibilityDuration = std::list<bool>(params.ap.nbIterPenaltyManagement, true);
   301	}
   302	
   303	Population::~Population()
   304	{
   305		for (int i = 0; i < (int)feasibleSubpop.size(); i++) delete feasibleSubpop[i];
   306		for (int i = 0; i < (int)infeasibleSubpop.size(); i++) delete infeasibleSubpop[i];
   307	}
```

</details>
<details>
<summary>Tool Call: Read file `Program/Genetic.h`</summary>

*Status: Completed*

```HGS-CVRP/Program/Genetic.h
     1	/*MIT License
     2	
     3	Copyright(c) 2020 Thibaut Vidal
     4	
     5	Permission is hereby granted, free of charge, to any person obtaining a copy
     6	of this software and associated documentation files(the "Software"), to deal
     7	in the Software without restriction, including without limitation the rights
     8	to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
     9	copies of the Software, and to permit persons to whom the Software is
    10	furnished to do so, subject to the following conditions :
    11	
    12	The above copyright notice and this permission notice shall be included in all
    13	copies or substantial portions of the Software.
    14	
    15	THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    16	IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    17	FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE
    18	AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    19	LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    20	OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    21	SOFTWARE.*/
    22	
    23	#ifndef GENETIC_H
    24	#define GENETIC_H
    25	
    26	#include "Population.h"
    27	#include "Individual.h"
    28	
    29	class Genetic
    30	{
    31	public:
    32	
    33		Params & params;				// Problem parameters
    34		Split split;					// Split algorithm
    35		LocalSearch localSearch;		// Local Search structure
    36		Population population;			// Population (public for now to give access to the solutions, but should be be improved later on)
    37		Individual offspring;			// First individual to be used as input for the crossover
    38	
    39		// OX Crossover
    40		void crossoverOX(Individual & result, const Individual & parent1, const Individual & parent2);
    41	
    42	    // Running the genetic algorithm until maxIterNonProd consecutive iterations or a time limit
    43	    void run() ;
    44	
    45		// Constructor
    46		Genetic(Params & params);
    47	};
    48	
    49	#endif

```

</details>
<details>
<summary>Tool Call: Read file `Program/Genetic.cpp`</summary>

*Status: Completed*

```HGS-CVRP/Program/Genetic.cpp
     1	#include "Genetic.h"
     2	
     3	void Genetic::run()
     4	{	
     5		/* INITIAL POPULATION */
     6		population.generatePopulation();
     7	
     8		int nbIter;
     9		int nbIterNonProd = 1;
    10		if (params.verbose) std::cout << "----- STARTING GENETIC ALGORITHM" << std::endl;
    11		for (nbIter = 0 ; nbIterNonProd <= params.ap.nbIter && (params.ap.timeLimit == 0 || (double)(clock()-params.startTime)/(double)CLOCKS_PER_SEC < params.ap.timeLimit) ; nbIter++)
    12		{	
    13			/* SELECTION AND CROSSOVER */
    14			crossoverOX(offspring, population.getBinaryTournament(),population.getBinaryTournament());
    15	
    16			/* LOCAL SEARCH */
    17			localSearch.run(offspring, params.penaltyCapacity, params.penaltyDuration);
    18			bool isNewBest = population.addIndividual(offspring,true);
    19			if (!offspring.eval.isFeasible && params.ran()%2 == 0) // Repair half of the solutions in case of infeasibility
    20			{
    21				localSearch.run(offspring, params.penaltyCapacity*10., params.penaltyDuration*10.);
    22				if (offspring.eval.isFeasible) isNewBest = (population.addIndividual(offspring,false) || isNewBest);
    23			}
    24	
    25			/* TRACKING THE NUMBER OF ITERATIONS SINCE LAST SOLUTION IMPROVEMENT */
    26			if (isNewBest) nbIterNonProd = 1;
    27			else nbIterNonProd ++ ;
    28	
    29			/* DIVERSIFICATION, PENALTY MANAGEMENT AND TRACES */
    30			if (nbIter % params.ap.nbIterPenaltyManagement == 0) population.managePenalties();
    31			if (nbIter % params.ap.nbIterTraces == 0) population.printState(nbIter, nbIterNonProd);
    32	
    33			/* FOR TESTS INVOLVING SUCCESSIVE RUNS UNTIL A TIME LIMIT: WE RESET THE ALGORITHM/POPULATION EACH TIME maxIterNonProd IS ATTAINED*/
    34			if (params.ap.timeLimit != 0 && nbIterNonProd == params.ap.nbIter)
    35			{
    36				population.restart();
    37				nbIterNonProd = 1;
    38			}
    39		}
    40		if (params.verbose) std::cout << "----- GENETIC ALGORITHM FINISHED AFTER " << nbIter << " ITERATIONS. TIME SPENT: " << (double)(clock() - params.startTime) / (double)CLOCKS_PER_SEC << std::endl;
    41	}
    42	
    43	void Genetic::crossoverOX(Individual & result, const Individual & parent1, const Individual & parent2)
    44	{
    45		// Frequency table to track the customers which have been already inserted
    46		std::vector <bool> freqClient = std::vector <bool> (params.nbClients + 1, false);
    47	
    48		// Picking the beginning and end of the crossover zone
    49		std::uniform_int_distribution<> distr(0, params.nbClients-1);
    50		int start = distr(params.ran);
    51		int end = distr(params.ran);
    52	
    53		// Avoid that start and end coincide by accident
    54		while (end == start) end = distr(params.ran);
    55	
    56		// Copy from start to end
    57		int j = start;
    58		while (j % params.nbClients != (end + 1) % params.nbClients)
    59		{
    60			result.chromT[j % params.nbClients] = parent1.chromT[j % params.nbClients];
    61			freqClient[result.chromT[j % params.nbClients]] = true;
    62			j++;
    63		}
    64	
    65		// Fill the remaining elements in the order given by the second parent
    66		for (int i = 1; i <= params.nbClients; i++)
    67		{
    68			int temp = parent2.chromT[(end + i) % params.nbClients];
    69			if (freqClient[temp] == false)
    70			{
    71				result.chromT[j % params.nbClients] = temp;
    72				j++;
    73			}
    74		}
    75	
    76		// Complete the individual with the Split algorithm
    77		split.generalSplit(result, parent1.eval.nbRoutes);
    78	}
    79	
    80	Genetic::Genetic(Params & params) : 
    81		params(params), 
    82		split(params),
    83		localSearch(params),
    84		population(params,this->split,this->localSearch),
    85		offspring(params){}
    86	

```

</details>
<details>
<summary>Tool Call: Read file `Program/Split.h`</summary>

*Status: Completed*

```HGS-CVRP/Program/Split.h
     1	/*MIT License
     2	
     3	Copyright(c) 2020 Thibaut Vidal
     4	
     5	Permission is hereby granted, free of charge, to any person obtaining a copy
     6	of this software and associated documentation files(the "Software"), to deal
     7	in the Software without restriction, including without limitation the rights
     8	to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
     9	copies of the Software, and to permit persons to whom the Software is
    10	furnished to do so, subject to the following conditions :
    11	
    12	The above copyright notice and this permission notice shall be included in all
    13	copies or substantial portions of the Software.
    14	
    15	THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    16	IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    17	FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE
    18	AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    19	LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    20	OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    21	SOFTWARE.*/
    22	
    23	#ifndef SPLIT_H
    24	#define SPLIT_H
    25	
    26	#include "Params.h"
    27	#include "Individual.h"
    28	
    29	struct ClientSplit
    30	{
    31		double demand;
    32		double serviceTime;
    33		double d0_x;
    34		double dx_0;
    35		double dnext;
    36		ClientSplit() : demand(0.), serviceTime(0.), d0_x(0.), dx_0(0.), dnext(0.) {};
    37	};
    38	
    39	// Simple Deque which is used for all Linear Split algorithms
    40	struct Trivial_Deque
    41	{
    42		std::vector <int> myDeque; // Simply a vector structure to keep the elements of the queue
    43		int indexFront; // Index of the front element
    44		int indexBack; // Index of the back element
    45		inline void pop_front(){indexFront++;} // Removes the front element of the queue D
    46		inline void pop_back(){indexBack--;} // Removes the back element of the queue D
    47		inline void push_back(int i){indexBack++; myDeque[indexBack] = i;} // Appends a new element to the back of the queue D
    48		inline int get_front(){return myDeque[indexFront];}
    49		inline int get_next_front(){return myDeque[indexFront + 1];}
    50		inline int get_back(){return myDeque[indexBack];}
    51		void reset(int firstNode) { myDeque[0] = firstNode; indexBack = 0; indexFront = 0; }
    52		inline int size(){return indexBack - indexFront + 1;}
    53		
    54		Trivial_Deque(int nbElements, int firstNode)
    55		{
    56			myDeque = std::vector <int>(nbElements);
    57			myDeque[0] = firstNode;
    58			indexBack = 0;
    59			indexFront = 0;
    60		}
    61	};
    62	
    63	class Split
    64	{
    65	
    66	 private:
    67	
    68	 // Problem parameters
    69	 const Params & params ;
    70	 int maxVehicles ;
    71	
    72	 /* Auxiliary data structures to run the Linear Split algorithm */
    73	 std::vector < ClientSplit > cliSplit;
    74	 std::vector < std::vector < double > > potential;  // Potential vector
    75	 std::vector < std::vector < int > > pred;  // Indice of the predecessor in an optimal path
    76	 std::vector <double> sumDistance; // sumDistance[i] for i > 1 contains the sum of distances : sum_{k=1}^{i-1} d_{k,k+1}
    77	 std::vector <double> sumLoad; // sumLoad[i] for i >= 1 contains the sum of loads : sum_{k=1}^{i} q_k
    78	 std::vector <double> sumService; // sumService[i] for i >= 1 contains the sum of service time : sum_{k=1}^{i} s_k
    79	
    80	 // To be called with i < j only
    81	 // Computes the cost of propagating the label i until j
    82	 inline double propagate(int i, int j, int k)
    83	 {
    84		 return potential[k][i] + sumDistance[j] - sumDistance[i + 1] + cliSplit[i + 1].d0_x + cliSplit[j].dx_0
    85			 + params.penaltyCapacity * std::max<double>(sumLoad[j] - sumLoad[i] - params.vehicleCapacity, 0.);
    86	 }
    87	
    88	 // Tests if i dominates j as a predecessor for all nodes x >= j+1
    89	 // We assume that i < j
    90	 inline bool dominates(int i, int j, int k)
    91	 {
    92		 return potential[k][j] + cliSplit[j + 1].d0_x > potential[k][i] + cliSplit[i + 1].d0_x + sumDistance[j + 1] - sumDistance[i + 1]
    93			 + params.penaltyCapacity * (sumLoad[j] - sumLoad[i]);
    94	 }
    95	
    96	 // Tests if j dominates i as a predecessor for all nodes x >= j+1
    97	 // We assume that i < j
    98	 inline bool dominatesRight(int i, int j, int k)
    99	 {
   100		 return potential[k][j] + cliSplit[j + 1].d0_x < potential[k][i] + cliSplit[i + 1].d0_x + sumDistance[j + 1] - sumDistance[i + 1] + MY_EPSILON;
   101	 }
   102	
   103	  // Split for unlimited fleet
   104	  int splitSimple(Individual & indiv);
   105	
   106	  // Split for limited fleet
   107	  int splitLF(Individual & indiv);
   108	
   109	public:
   110	
   111	  // General Split function (tests the unlimited fleet, and only if it does not produce a feasible solution, runs the Split algorithm for limited fleet)
   112	  void generalSplit(Individual & indiv, int nbMaxVehicles);
   113	
   114	  // Constructor
   115	  Split(const Params & params);
   116	
   117	};
   118	#endif

```

</details>
<details>
<summary>Tool Call: Read file `Program/Split.cpp`</summary>

*Status: Completed*

```HGS-CVRP/Program/Split.cpp
     1	#include "Split.h" 
     2	
     3	void Split::generalSplit(Individual & indiv, int nbMaxVehicles)
     4	{
     5		// Do not apply Split with fewer vehicles than the trivial (LP) bin packing bound
     6		maxVehicles = std::max<int>(nbMaxVehicles, std::ceil(params.totalDemand/params.vehicleCapacity));
     7	
     8		// Initialization of the data structures for the linear split algorithms
     9		// Direct application of the code located at https://github.com/vidalt/Split-Library
    10		for (int i = 1; i <= params.nbClients; i++)
    11		{
    12			cliSplit[i].demand = params.cli[indiv.chromT[i - 1]].demand;
    13			cliSplit[i].serviceTime = params.cli[indiv.chromT[i - 1]].serviceDuration;
    14			cliSplit[i].d0_x = params.timeCost[0][indiv.chromT[i - 1]];
    15			cliSplit[i].dx_0 = params.timeCost[indiv.chromT[i - 1]][0];
    16			if (i < params.nbClients) cliSplit[i].dnext = params.timeCost[indiv.chromT[i - 1]][indiv.chromT[i]];
    17			else cliSplit[i].dnext = -1.e30;
    18			sumLoad[i] = sumLoad[i - 1] + cliSplit[i].demand;
    19			sumService[i] = sumService[i - 1] + cliSplit[i].serviceTime;
    20			sumDistance[i] = sumDistance[i - 1] + cliSplit[i - 1].dnext;
    21		}
    22	
    23		// We first try the simple split, and then the Split with limited fleet if this is not successful
    24		if (splitSimple(indiv) == 0)
    25			splitLF(indiv);
    26	
    27		// Build up the rest of the Individual structure
    28		indiv.evaluateCompleteCost(params);
    29	}
    30	
    31	int Split::splitSimple(Individual & indiv)
    32	{
    33		// Reinitialize the potential structures
    34		potential[0][0] = 0;
    35		for (int i = 1; i <= params.nbClients; i++)
    36			potential[0][i] = 1.e30;
    37	
    38		// MAIN ALGORITHM -- Simple Split using Bellman's algorithm in topological order
    39		// This code has been maintained as it is very simple and can be easily adapted to a variety of constraints, whereas the O(n) Split has a more restricted application scope
    40		if (params.isDurationConstraint)
    41		{
    42			for (int i = 0; i < params.nbClients; i++)
    43			{
    44				double load = 0.;
    45				double distance = 0.;
    46				double serviceDuration = 0.;
    47				for (int j = i + 1; j <= params.nbClients && load <= 1.5 * params.vehicleCapacity ; j++)
    48				{
    49					load += cliSplit[j].demand;
    50					serviceDuration += cliSplit[j].serviceTime;
    51					if (j == i + 1) distance += cliSplit[j].d0_x;
    52					else distance += cliSplit[j - 1].dnext;
    53					double cost = distance + cliSplit[j].dx_0
    54						+ params.penaltyCapacity * std::max<double>(load - params.vehicleCapacity, 0.)
    55						+ params.penaltyDuration * std::max<double>(distance + cliSplit[j].dx_0 + serviceDuration - params.durationLimit, 0.);
    56					if (potential[0][i] + cost < potential[0][j])
    57					{
    58						potential[0][j] = potential[0][i] + cost;
    59						pred[0][j] = i;
    60					}
    61				}
    62			}
    63		}
    64		else
    65		{
    66			Trivial_Deque queue = Trivial_Deque(params.nbClients + 1, 0);
    67			for (int i = 1; i <= params.nbClients; i++)
    68			{
    69				// The front is the best predecessor for i
    70				potential[0][i] = propagate(queue.get_front(), i, 0);
    71				pred[0][i] = queue.get_front();
    72	
    73				if (i < params.nbClients)
    74				{
    75					// If i is not dominated by the last of the pile
    76					if (!dominates(queue.get_back(), i, 0))
    77					{
    78						// then i will be inserted, need to remove whoever is dominated by i.
    79						while (queue.size() > 0 && dominatesRight(queue.get_back(), i, 0))
    80							queue.pop_back();
    81						queue.push_back(i);
    82					}
    83					// Check iteratively if front is dominated by the next front
    84					while (queue.size() > 1 && propagate(queue.get_front(), i + 1, 0) > propagate(queue.get_next_front(), i + 1, 0) - MY_EPSILON)
    85						queue.pop_front();
    86				}
    87			}
    88		}
    89	
    90		if (potential[0][params.nbClients] > 1.e29)
    91			throw std::string("ERROR : no Split solution has been propagated until the last node");
    92	
    93		// Filling the chromR structure
    94		for (int k = params.nbVehicles - 1; k >= maxVehicles; k--)
    95			indiv.chromR[k].clear();
    96	
    97		int end = params.nbClients;
    98		for (int k = maxVehicles - 1; k >= 0; k--)
    99		{
   100			indiv.chromR[k].clear();
   101			int begin = pred[0][end];
   102			for (int ii = begin; ii < end; ii++)
   103				indiv.chromR[k].push_back(indiv.chromT[ii]);
   104			end = begin;
   105		}
   106	
   107		// Return OK in case the Split algorithm reached the beginning of the routes
   108		return (end == 0);
   109	}
   110	
   111	// Split for problems with limited fleet
   112	int Split::splitLF(Individual & indiv)
   113	{
   114		// Initialize the potential structures
   115		potential[0][0] = 0;
   116		for (int k = 0; k <= maxVehicles; k++)
   117			for (int i = 1; i <= params.nbClients; i++)
   118				potential[k][i] = 1.e30;
   119	
   120		// MAIN ALGORITHM -- Simple Split using Bellman's algorithm in topological order
   121		// This code has been maintained as it is very simple and can be easily adapted to a variety of constraints, whereas the O(n) Split has a more restricted application scope
   122		if (params.isDurationConstraint) 
   123		{
   124			for (int k = 0; k < maxVehicles; k++)
   125			{
   126				for (int i = k; i < params.nbClients && potential[k][i] < 1.e29 ; i++)
   127				{
   128					double load = 0.;
   129					double serviceDuration = 0.;
   130					double distance = 0.;
   131					for (int j = i + 1; j <= params.nbClients && load <= 1.5 * params.vehicleCapacity ; j++) // Setting a maximum limit on load infeasibility to accelerate the algorithm
   132					{
   133						load += cliSplit[j].demand;
   134						serviceDuration += cliSplit[j].serviceTime;
   135						if (j == i + 1) distance += cliSplit[j].d0_x;
   136						else distance += cliSplit[j - 1].dnext;
   137						double cost = distance + cliSplit[j].dx_0
   138									+ params.penaltyCapacity * std::max<double>(load - params.vehicleCapacity, 0.)
   139									+ params.penaltyDuration * std::max<double>(distance + cliSplit[j].dx_0 + serviceDuration - params.durationLimit, 0.);
   140						if (potential[k][i] + cost < potential[k + 1][j])
   141						{
   142							potential[k + 1][j] = potential[k][i] + cost;
   143							pred[k + 1][j] = i;
   144						}
   145					}
   146				}
   147			}
   148		}
   149		else // MAIN ALGORITHM -- Without duration constraints in O(n), from "Vidal, T. (2016). Split algorithm in O(n) for the capacitated vehicle routing problem. C&OR"
   150		{
   151			Trivial_Deque queue = Trivial_Deque(params.nbClients + 1, 0);
   152			for (int k = 0; k < maxVehicles; k++)
   153			{
   154				// in the Split problem there is always one feasible solution with k routes that reaches the index k in the tour.
   155				queue.reset(k);
   156	
   157				// The range of potentials < 1.29 is always an interval.
   158				// The size of the queue will stay >= 1 until we reach the end of this interval.
   159				for (int i = k + 1; i <= params.nbClients && queue.size() > 0; i++)
   160				{
   161					// The front is the best predecessor for i
   162					potential[k + 1][i] = propagate(queue.get_front(), i, k);
   163					pred[k + 1][i] = queue.get_front();
   164	
   165					if (i < params.nbClients)
   166					{
   167						// If i is not dominated by the last of the pile 
   168						if (!dominates(queue.get_back(), i, k))
   169						{
   170							// then i will be inserted, need to remove whoever he dominates
   171							while (queue.size() > 0 && dominatesRight(queue.get_back(), i, k))
   172								queue.pop_back();
   173							queue.push_back(i);
   174						}
   175	
   176						// Check iteratively if front is dominated by the next front
   177						while (queue.size() > 1 && propagate(queue.get_front(), i + 1, k) > propagate(queue.get_next_front(), i + 1, k) - MY_EPSILON)
   178							queue.pop_front();
   179					}
   180				}
   181			}
   182		}
   183	
   184		if (potential[maxVehicles][params.nbClients] > 1.e29)
   185			throw std::string("ERROR : no Split solution has been propagated until the last node");
   186	
   187		// It could be cheaper to use a smaller number of vehicles
   188		double minCost = potential[maxVehicles][params.nbClients];
   189		int nbRoutes = maxVehicles;
   190		for (int k = 1; k < maxVehicles; k++)
   191			if (potential[k][params.nbClients] < minCost)
   192				{minCost = potential[k][params.nbClients]; nbRoutes = k;}
   193	
   194		// Filling the chromR structure
   195		for (int k = params.nbVehicles-1; k >= nbRoutes ; k--)
   196			indiv.chromR[k].clear();
   197	
   198		int end = params.nbClients;
   199		for (int k = nbRoutes - 1; k >= 0; k--)
   200		{
   201			indiv.chromR[k].clear();
   202			int begin = pred[k+1][end];
   203			for (int ii = begin; ii < end; ii++)
   204				indiv.chromR[k].push_back(indiv.chromT[ii]);
   205			end = begin;
   206		}
   207	
   208		// Return OK in case the Split algorithm reached the beginning of the routes
   209		return (end == 0);
   210	}
   211	
   212	Split::Split(const Params & params): params(params)
   213	{
   214		// Structures of the linear Split
   215		cliSplit = std::vector <ClientSplit>(params.nbClients + 1);
   216		sumDistance = std::vector <double>(params.nbClients + 1,0.);
   217		sumLoad = std::vector <double>(params.nbClients + 1,0.);
   218		sumService = std::vector <double>(params.nbClients + 1, 0.);
   219		potential = std::vector < std::vector <double> >(params.nbVehicles + 1, std::vector <double>(params.nbClients + 1,1.e30));
   220		pred = std::vector < std::vector <int> >(params.nbVehicles + 1, std::vector <int>(params.nbClients + 1,0));
   221	}

```

</details>
<details>
<summary>Tool Call: Read file `Program/LocalSearch.h`</summary>

*Status: Completed*

```HGS-CVRP/Program/LocalSearch.h
     1	/*MIT License
     2	
     3	Copyright(c) 2020 Thibaut Vidal
     4	
     5	Permission is hereby granted, free of charge, to any person obtaining a copy
     6	of this software and associated documentation files(the "Software"), to deal
     7	in the Software without restriction, including without limitation the rights
     8	to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
     9	copies of the Software, and to permit persons to whom the Software is
    10	furnished to do so, subject to the following conditions :
    11	
    12	The above copyright notice and this permission notice shall be included in all
    13	copies or substantial portions of the Software.
    14	
    15	THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    16	IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    17	FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE
    18	AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    19	LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    20	OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    21	SOFTWARE.*/
    22	
    23	#ifndef LOCALSEARCH_H
    24	#define LOCALSEARCH_H
    25	
    26	#include "Individual.h"
    27	
    28	struct Node ;
    29	
    30	// Structure containing a route
    31	struct Route
    32	{
    33		int cour;							// Route index
    34		int nbCustomers;					// Number of customers visited in the route
    35		int whenLastModified;				// "When" this route has been last modified
    36		int whenLastTestedSWAPStar;			// "When" the SWAP* moves for this route have been last tested
    37		Node * depot;						// Pointer to the associated depot
    38		double duration;					// Total time on the route
    39		double load;						// Total load on the route
    40		double reversalDistance;			// Difference of cost if the route is reversed
    41		double penalty;						// Current sum of load and duration penalties
    42		double polarAngleBarycenter;		// Polar angle of the barycenter of the route
    43		CircleSector sector;				// Circle sector associated to the set of customers
    44	};
    45	
    46	struct Node
    47	{
    48		bool isDepot;						// Tells whether this node represents a depot or not
    49		int cour;							// Node index
    50		int position;						// Position in the route
    51		int whenLastTestedRI;				// "When" the RI moves for this node have been last tested
    52		Node * next;						// Next node in the route order
    53		Node * prev;						// Previous node in the route order
    54		Route * route;						// Pointer towards the associated route
    55		double cumulatedLoad;				// Cumulated load on this route until the customer (including itself)
    56		double cumulatedTime;				// Cumulated time on this route until the customer (including itself)
    57		double cumulatedReversalDistance;	// Difference of cost if the segment of route (0...cour) is reversed (useful for 2-opt moves with asymmetric problems)
    58		double deltaRemoval;				// Difference of cost in the current route if the node is removed (used in SWAP*)
    59	};
    60	
    61	// Structure used in SWAP* to remember the three best insertion positions of a customer in a given route
    62	struct ThreeBestInsert
    63	{
    64		int whenLastCalculated;
    65		double bestCost[3];
    66		Node * bestLocation[3];
    67	
    68		void compareAndAdd(double costInsert, Node * placeInsert)
    69		{
    70			if (costInsert >= bestCost[2]) return;
    71			else if (costInsert >= bestCost[1])
    72			{
    73				bestCost[2] = costInsert; bestLocation[2] = placeInsert;
    74			}
    75			else if (costInsert >= bestCost[0])
    76			{
    77				bestCost[2] = bestCost[1]; bestLocation[2] = bestLocation[1];
    78				bestCost[1] = costInsert; bestLocation[1] = placeInsert;
    79			}
    80			else
    81			{
    82				bestCost[2] = bestCost[1]; bestLocation[2] = bestLocation[1];
    83				bestCost[1] = bestCost[0]; bestLocation[1] = bestLocation[0];
    84				bestCost[0] = costInsert; bestLocation[0] = placeInsert;
    85			}
    86		}
    87	
    88		// Resets the structure (no insertion calculated)
    89		void reset()
    90		{
    91			bestCost[0] = 1.e30; bestLocation[0] = NULL;
    92			bestCost[1] = 1.e30; bestLocation[1] = NULL;
    93			bestCost[2] = 1.e30; bestLocation[2] = NULL;
    94		}
    95	
    96		ThreeBestInsert() { reset(); };
    97	};
    98	
    99	// Structured used to keep track of the best SWAP* move
   100	struct SwapStarElement
   101	{
   102		double moveCost = 1.e30 ;
   103		Node * U = NULL ;
   104		Node * bestPositionU = NULL;
   105		Node * V = NULL;
   106		Node * bestPositionV = NULL;
   107	};
   108	
   109	// Main local learch structure
   110	class LocalSearch
   111	{
   112	
   113	private:
   114		
   115		Params & params ;							// Problem parameters
   116		bool searchCompleted;						// Tells whether all moves have been evaluated without success
   117		int nbMoves;								// Total number of moves (RI and SWAP*) applied during the local search. Attention: this is not only a simple counter, it is also used to avoid repeating move evaluations
   118		std::vector < int > orderNodes;				// Randomized order for checking the nodes in the RI local search
   119		std::vector < int > orderRoutes;			// Randomized order for checking the routes in the SWAP* local search
   120		std::set < int > emptyRoutes;				// indices of all empty routes
   121		int loopID;									// Current loop index
   122	
   123		/* THE SOLUTION IS REPRESENTED AS A LINKED LIST OF ELEMENTS */
   124		std::vector < Node > clients;				// Elements representing clients (clients[0] is a sentinel and should not be accessed)
   125		std::vector < Node > depots;				// Elements representing depots
   126		std::vector < Node > depotsEnd;				// Duplicate of the depots to mark the end of the routes
   127		std::vector < Route > routes;				// Elements representing routes
   128		std::vector < std::vector < ThreeBestInsert > > bestInsertClient;   // (SWAP*) For each route and node, storing the cheapest insertion cost 
   129	
   130		/* TEMPORARY VARIABLES USED IN THE LOCAL SEARCH LOOPS */
   131		// nodeUPrev -> nodeU -> nodeX -> nodeXNext
   132		// nodeVPrev -> nodeV -> nodeY -> nodeYNext
   133		Node * nodeU ;
   134		Node * nodeX ;
   135	    Node * nodeV ;
   136		Node * nodeY ;
   137		Route * routeU ;
   138		Route * routeV ;
   139		int nodeUPrevIndex, nodeUIndex, nodeXIndex, nodeXNextIndex ;	
   140		int nodeVPrevIndex, nodeVIndex, nodeYIndex, nodeYNextIndex ;	
   141		double loadU, loadX, loadV, loadY;
   142		double serviceU, serviceX, serviceV, serviceY;
   143		double penaltyCapacityLS, penaltyDurationLS ;
   144		bool intraRouteMove ;
   145	
   146		void setLocalVariablesRouteU(); // Initializes some local variables and distances associated to routeU to avoid always querying the same values in the distance matrix
   147		void setLocalVariablesRouteV(); // Initializes some local variables and distances associated to routeV to avoid always querying the same values in the distance matrix
   148	
   149		inline double penaltyExcessDuration(double myDuration) {return std::max<double>(0., myDuration - params.durationLimit)*penaltyDurationLS;}
   150		inline double penaltyExcessLoad(double myLoad) {return std::max<double>(0., myLoad - params.vehicleCapacity)*penaltyCapacityLS;}
   151	
   152		/* RELOCATE MOVES */
   153		// (Legacy notations: move1...move9 from Prins 2004)
   154		bool move1(); // If U is a client node, remove U and insert it after V
   155		bool move2(); // If U and X are client nodes, remove them and insert (U,X) after V
   156		bool move3(); // If U and X are client nodes, remove them and insert (X,U) after V
   157	
   158		/* SWAP MOVES */
   159		bool move4(); // If U and V are client nodes, swap U and V
   160		bool move5(); // If U, X and V are client nodes, swap (U,X) and V
   161		bool move6(); // If (U,X) and (V,Y) are client nodes, swap (U,X) and (V,Y) 
   162		 
   163		/* 2-OPT and 2-OPT* MOVES */
   164		bool move7(); // If route(U) == route(V), replace (U,X) and (V,Y) by (U,V) and (X,Y)
   165		bool move8(); // If route(U) != route(V), replace (U,X) and (V,Y) by (U,V) and (X,Y)
   166		bool move9(); // If route(U) != route(V), replace (U,X) and (V,Y) by (U,Y) and (V,X)
   167	
   168		/* SUB-ROUTINES FOR EFFICIENT SWAP* EVALUATIONS */
   169		bool swapStar(); // Calculates all SWAP* between routeU and routeV and apply the best improving move
   170		double getCheapestInsertSimultRemoval(Node * U, Node * V, Node *& bestPosition); // Calculates the insertion cost and position in the route of V, where V is omitted
   171		void preprocessInsertions(Route * R1, Route * R2); // Preprocess all insertion costs of nodes of route R1 in route R2
   172	
   173		/* ROUTINES TO UPDATE THE SOLUTIONS */
   174		static void insertNode(Node * U, Node * V);		// Solution update: Insert U after V
   175		static void swapNode(Node * U, Node * V) ;		// Solution update: Swap U and V							   
   176		void updateRouteData(Route * myRoute);			// Updates the preprocessed data of a route
   177	
   178		public:
   179	
   180		// Run the local search with the specified penalty values
   181		void run(Individual & indiv, double penaltyCapacityLS, double penaltyDurationLS);
   182	
   183		// Loading an initial solution into the local search
   184		void loadIndividual(const Individual & indiv);
   185	
   186		// Exporting the LS solution into an individual and calculating the penalized cost according to the original penalty weights from Params
   187		void exportIndividual(Individual & indiv);
   188	
   189		// Constructor
   190		LocalSearch(Params & params);
   191	};
   192	
   193	#endif

```

</details>
<details>
<summary>Tool Call: Read file `Program/LocalSearch.cpp`</summary>

*Status: Completed*

```
SUCCESS: File outline retrieved. This file is too large to read all at once, so the outline below shows the file's structure with line numbers.

IMPORTANT: Do NOT retry this call without line numbers - you will get the same outline.
Instead, use the line numbers below to read specific sections by calling this tool again with start_line and end_line parameters.

# File outline for /home/username/HGS-CVRP/Program/LocalSearch.cpp

void LocalSearch::run( ) [L3-96]
 int i [L12]
 int posU [L23]
 int lastTestRINodeU [L26]
 int posV [L28]
 int rU [L76]
 int lastTestSWAPStarRouteU [L79]
 int rV [L81]
void LocalSearch::setLocalVariablesRouteU() [L98-110]
void LocalSearch::setLocalVariablesRouteV() [L112-125]
bool LocalSearch::move1() [L127-155]
 double costSuppU [L129]
 double costSuppV [L130]
bool LocalSearch::move2() [L157-186]
 double costSuppU [L159]
 double costSuppV [L160]
bool LocalSearch::move3() [L188-217]
 double costSuppU [L190]
 double costSuppV [L191]
bool LocalSearch::move4() [L219-247]
 double costSuppU [L221]
 double costSuppV [L222]
bool LocalSearch::move5() [L249-278]
 double costSuppU [L251]
 double costSuppV [L252]
bool LocalSearch::move6() [L280-309]
 double costSuppU [L282]
 double costSuppV [L283]
bool LocalSearch::move7() [L311-341]
 double cost [L315]
 Node * nodeNum [L320]
 Node * temp [L326]
bool LocalSearch::move8() [L343-421]
 double cost [L345-347]
 Node * depotU [L359]
 Node * depotV [L360]
 Node * depotUFin [L361]
 Node * depotVFin [L362]
 Node * depotVSuiv [L363]
 Node * xx [L366]
 Node * vv [L367]
bool LocalSearch::move9() [L423-483]
 double cost [L425-426]
 Node * depotU [L438]
 Node * depotV [L439]
 Node * depotUFin [L440]
 Node * depotVFin [L441]
 Node * depotUpred [L442]
 Node * count [L444]
bool LocalSearch::swapStar() [L485-572]
 SwapStarElement myBestSwapStar [L487]
 double deltaPenRouteU [L498]
 double deltaPenRouteV [L499]
 SwapStarElement mySwapStar [L504]
 double extraV [L509]
 double extraU [L512]
 SwapStarElement mySwapStar [L529]
 double deltaDistRouteU [L532]
 double deltaDistRouteV [L533]
 SwapStarElement mySwapStar [L547]
 double deltaDistRouteU [L550]
 double deltaDistRouteV [L551]
double LocalSearch::getCheapestInsertSimultRemoval( ) [L574-605]
 ThreeBestInsert * myBestInsert [L576]
 bool found [L577]
 double bestCost [L581]
 double deltaCost [L597]
void LocalSearch::preprocessInsertions( ) [L607-626]
 Node * U [L609]
 Node * V [L619]
 double deltaCost [L621]
void LocalSearch::insertNode( ) [L628-637]
void LocalSearch::swapNode( ) [L639-660]
 Node * myVPred [L641]
 Node * myVSuiv [L642]
 Node * myUPred [L643]
 Node * myUSuiv [L644]
 Route * myRouteU [L645]
 Route * myRouteV [L646]
void LocalSearch::updateRouteData( ) [L662-717]
 int myplace [L664]
 double myload [L665]
 double mytime [L666]
 double myReversalDistance [L667]
 double cumulatedX [L668]
 double cumulatedY [L669]
 Node * mynode [L671]
 bool firstIt [L677]
void LocalSearch::loadIndividual( ) [L719-760]
 int r [L723]
 Node * myDepot [L725]
 Node * myDepotFin [L726]
 Route * myRoute [L727]
 Node * myClient [L732]
 int i [L736]
 Node * myClientPred [L738]
 int i [L754]
 int i [L758]
void LocalSearch::exportIndividual( ) [L762-784]
 std::vector < std::pair <double, int> > routePolarAngles [L764]
 int r [L765]
 int pos [L769]
 int r [L770]
 Node * node [L773]
LocalSearch::LocalSearch( ) [L786-812]
 int i [L794]
 int i [L799]
 int i [L810]
 int r [L811]

Showing symbols 1-109 (total symbols: 109)


NEXT STEPS: To read a specific symbol's implementation, call read_file with the same path plus start_line and end_line from the outline above.
For example, to read a function shown as [L100-150], use start_line: 100 and end_line: 150.
```

</details>
<details>
<summary>Tool Call: Read file `Program/LocalSearch.cpp` (lines 1-250)</summary>

*Status: Completed*

```HGS-CVRP/Program/LocalSearch.cpp
     1	#include "LocalSearch.h" 
     2	
     3	void LocalSearch::run(Individual & indiv, double penaltyCapacityLS, double penaltyDurationLS)
     4	{
     5		this->penaltyCapacityLS = penaltyCapacityLS;
     6		this->penaltyDurationLS = penaltyDurationLS;
     7		loadIndividual(indiv);
     8	
     9		// Shuffling the order of the nodes explored by the LS to allow for more diversity in the search
    10		std::shuffle(orderNodes.begin(), orderNodes.end(), params.ran);
    11		std::shuffle(orderRoutes.begin(), orderRoutes.end(), params.ran);
    12		for (int i = 1; i <= params.nbClients; i++)
    13			if (params.ran() % params.ap.nbGranular == 0)  // O(n/nbGranular) calls to the inner function on average, to achieve linear-time complexity overall
    14				std::shuffle(params.correlatedVertices[i].begin(), params.correlatedVertices[i].end(), params.ran);
    15	
    16		searchCompleted = false;
    17		for (loopID = 0; !searchCompleted; loopID++)
    18		{
    19			if (loopID > 1) // Allows at least two loops since some moves involving empty routes are not checked at the first loop
    20				searchCompleted = true;
    21	
    22			/* CLASSICAL ROUTE IMPROVEMENT (RI) MOVES SUBJECT TO A PROXIMITY RESTRICTION */
    23			for (int posU = 0; posU < params.nbClients; posU++)
    24			{
    25				nodeU = &clients[orderNodes[posU]];
    26				int lastTestRINodeU = nodeU->whenLastTestedRI;
    27				nodeU->whenLastTestedRI = nbMoves;
    28				for (int posV = 0; posV < (int)params.correlatedVertices[nodeU->cour].size(); posV++)
    29				{
    30					nodeV = &clients[params.correlatedVertices[nodeU->cour][posV]];
    31					if (loopID == 0 || std::max<int>(nodeU->route->whenLastModified, nodeV->route->whenLastModified) > lastTestRINodeU) // only evaluate moves involving routes that have been modified since last move evaluations for nodeU
    32					{
    33						// Randomizing the order of the neighborhoods within this loop does not matter much as we are already randomizing the order of the node pairs (and it's not very common to find improving moves of different types for the same node pair)
    34						setLocalVariablesRouteU();
    35						setLocalVariablesRouteV();
    36						if (move1()) continue; // RELOCATE
    37						if (move2()) continue; // RELOCATE
    38						if (move3()) continue; // RELOCATE
    39						if (nodeUIndex <= nodeVIndex && move4()) continue; // SWAP
    40						if (move5()) continue; // SWAP
    41						if (nodeUIndex <= nodeVIndex && move6()) continue; // SWAP
    42						if (intraRouteMove && move7()) continue; // 2-OPT
    43						if (!intraRouteMove && move8()) continue; // 2-OPT*
    44						if (!intraRouteMove && move9()) continue; // 2-OPT*
    45	
    46						// Trying moves that insert nodeU directly after the depot
    47						if (nodeV->prev->isDepot)
    48						{
    49							nodeV = nodeV->prev;
    50							setLocalVariablesRouteV();
    51							if (move1()) continue; // RELOCATE
    52							if (move2()) continue; // RELOCATE
    53							if (move3()) continue; // RELOCATE
    54							if (!intraRouteMove && move8()) continue; // 2-OPT*
    55							if (!intraRouteMove && move9()) continue; // 2-OPT*
    56						}
    57					}
    58				}
    59	
    60				/* MOVES INVOLVING AN EMPTY ROUTE -- NOT TESTED IN THE FIRST LOOP TO AVOID INCREASING TOO MUCH THE FLEET SIZE */
    61				if (loopID > 0 && !emptyRoutes.empty())
    62				{
    63					nodeV = routes[*emptyRoutes.begin()].depot;
    64					setLocalVariablesRouteU();
    65					setLocalVariablesRouteV();
    66					if (move1()) continue; // RELOCATE
    67					if (move2()) continue; // RELOCATE
    68					if (move3()) continue; // RELOCATE
    69					if (move9()) continue; // 2-OPT*
    70				}
    71			}
    72	
    73			if (params.ap.useSwapStar == 1 && params.areCoordinatesProvided)
    74			{
    75				/* (SWAP*) MOVES LIMITED TO ROUTE PAIRS WHOSE CIRCLE SECTORS OVERLAP */
    76				for (int rU = 0; rU < params.nbVehicles; rU++)
    77				{
    78					routeU = &routes[orderRoutes[rU]];
    79					int lastTestSWAPStarRouteU = routeU->whenLastTestedSWAPStar;
    80					routeU->whenLastTestedSWAPStar = nbMoves;
    81					for (int rV = 0; rV < params.nbVehicles; rV++)
    82					{
    83						routeV = &routes[orderRoutes[rV]];
    84						if (routeU->nbCustomers > 0 && routeV->nbCustomers > 0 && routeU->cour < routeV->cour
    85							&& (loopID == 0 || std::max<int>(routeU->whenLastModified, routeV->whenLastModified)
    86								> lastTestSWAPStarRouteU))
    87							if (CircleSector::overlap(routeU->sector, routeV->sector))
    88								swapStar();
    89					}
    90				}
    91			}
    92		}
    93	
    94		// Register the solution produced by the LS in the individual
    95		exportIndividual(indiv);
    96	}
    97	
    98	void LocalSearch::setLocalVariablesRouteU()
    99	{
   100		routeU = nodeU->route;
   101		nodeX = nodeU->next;
   102		nodeXNextIndex = nodeX->next->cour;
   103		nodeUIndex = nodeU->cour;
   104		nodeUPrevIndex = nodeU->prev->cour;
   105		nodeXIndex = nodeX->cour;
   106		loadU    = params.cli[nodeUIndex].demand;
   107		serviceU = params.cli[nodeUIndex].serviceDuration;
   108		loadX	 = params.cli[nodeXIndex].demand;
   109		serviceX = params.cli[nodeXIndex].serviceDuration;
   110	}
   111	
   112	void LocalSearch::setLocalVariablesRouteV()
   113	{
   114		routeV = nodeV->route;
   115		nodeY = nodeV->next;
   116		nodeYNextIndex = nodeY->next->cour;
   117		nodeVIndex = nodeV->cour;
   118		nodeVPrevIndex = nodeV->prev->cour;
   119		nodeYIndex = nodeY->cour;
   120		loadV    = params.cli[nodeVIndex].demand;
   121		serviceV = params.cli[nodeVIndex].serviceDuration;
   122		loadY	 = params.cli[nodeYIndex].demand;
   123		serviceY = params.cli[nodeYIndex].serviceDuration;
   124		intraRouteMove = (routeU == routeV);
   125	}
   126	
   127	bool LocalSearch::move1()
   128	{
   129		double costSuppU = params.timeCost[nodeUPrevIndex][nodeXIndex] - params.timeCost[nodeUPrevIndex][nodeUIndex] - params.timeCost[nodeUIndex][nodeXIndex];
   130		double costSuppV = params.timeCost[nodeVIndex][nodeUIndex] + params.timeCost[nodeUIndex][nodeYIndex] - params.timeCost[nodeVIndex][nodeYIndex];
   131	
   132		if (!intraRouteMove)
   133		{
   134			// Early move pruning to save CPU time. Guarantees that this move cannot improve without checking additional (load, duration...) constraints
   135			if (costSuppU + costSuppV >= routeU->penalty + routeV->penalty) return false;
   136	
   137			costSuppU += penaltyExcessDuration(routeU->duration + costSuppU - serviceU)
   138				+ penaltyExcessLoad(routeU->load - loadU)
   139				- routeU->penalty;
   140	
   141			costSuppV += penaltyExcessDuration(routeV->duration + costSuppV + serviceU)
   142				+ penaltyExcessLoad(routeV->load + loadU)
   143				- routeV->penalty;
   144		}
   145	
   146		if (costSuppU + costSuppV > -MY_EPSILON) return false;
   147		if (nodeUIndex == nodeYIndex) return false;
   148	
   149		insertNode(nodeU, nodeV);
   150		nbMoves++; // Increment move counter before updating route data
   151		searchCompleted = false;
   152		updateRouteData(routeU);
   153		if (!intraRouteMove) updateRouteData(routeV);
   154		return true;
   155	}
   156	
   157	bool LocalSearch::move2()
   158	{
   159		double costSuppU = params.timeCost[nodeUPrevIndex][nodeXNextIndex] - params.timeCost[nodeUPrevIndex][nodeUIndex] - params.timeCost[nodeXIndex][nodeXNextIndex];
   160		double costSuppV = params.timeCost[nodeVIndex][nodeUIndex] + params.timeCost[nodeXIndex][nodeYIndex] - params.timeCost[nodeVIndex][nodeYIndex];
   161	
   162		if (!intraRouteMove)
   163		{
   164			// Early move pruning to save CPU time. Guarantees that this move cannot improve without checking additional (load, duration...) constraints
   165			if (costSuppU + costSuppV >= routeU->penalty + routeV->penalty) return false;
   166	
   167			costSuppU += penaltyExcessDuration(routeU->duration + costSuppU - params.timeCost[nodeUIndex][nodeXIndex] - serviceU - serviceX)
   168				+ penaltyExcessLoad(routeU->load - loadU - loadX)
   169				- routeU->penalty;
   170	
   171			costSuppV += penaltyExcessDuration(routeV->duration + costSuppV + params.timeCost[nodeUIndex][nodeXIndex] + serviceU + serviceX)
   172				+ penaltyExcessLoad(routeV->load + loadU + loadX)
   173				- routeV->penalty;
   174		}
   175	
   176		if (costSuppU + costSuppV > -MY_EPSILON) return false;
   177		if (nodeU == nodeY || nodeV == nodeX || nodeX->isDepot) return false;
   178	
   179		insertNode(nodeU, nodeV);
   180		insertNode(nodeX, nodeU);
   181		nbMoves++; // Increment move counter before updating route data
   182		searchCompleted = false;
   183		updateRouteData(routeU);
   184		if (!intraRouteMove) updateRouteData(routeV);
   185		return true;
   186	}
   187	
   188	bool LocalSearch::move3()
   189	{
   190		double costSuppU = params.timeCost[nodeUPrevIndex][nodeXNextIndex] - params.timeCost[nodeUPrevIndex][nodeUIndex] - params.timeCost[nodeUIndex][nodeXIndex] - params.timeCost[nodeXIndex][nodeXNextIndex];
   191		double costSuppV = params.timeCost[nodeVIndex][nodeXIndex] + params.timeCost[nodeXIndex][nodeUIndex] + params.timeCost[nodeUIndex][nodeYIndex] - params.timeCost[nodeVIndex][nodeYIndex];
   192	
   193		if (!intraRouteMove)
   194		{
   195			// Early move pruning to save CPU time. Guarantees that this move cannot improve without checking additional (load, duration...) constraints
   196			if (costSuppU + costSuppV >= routeU->penalty + routeV->penalty) return false;
   197	
   198			costSuppU += penaltyExcessDuration(routeU->duration + costSuppU - serviceU - serviceX)
   199				+ penaltyExcessLoad(routeU->load - loadU - loadX)
   200				- routeU->penalty;
   201	
   202			costSuppV += penaltyExcessDuration(routeV->duration + costSuppV + serviceU + serviceX)
   203				+ penaltyExcessLoad(routeV->load + loadU + loadX)
   204				- routeV->penalty;
   205		}
   206	
   207		if (costSuppU + costSuppV > -MY_EPSILON) return false;
   208		if (nodeU == nodeY || nodeX == nodeV || nodeX->isDepot) return false;
   209	
   210		insertNode(nodeX, nodeV);
   211		insertNode(nodeU, nodeX);
   212		nbMoves++; // Increment move counter before updating route data
   213		searchCompleted = false;
   214		updateRouteData(routeU);
   215		if (!intraRouteMove) updateRouteData(routeV);
   216		return true;
   217	}
   218	
   219	bool LocalSearch::move4()
   220	{
   221		double costSuppU = params.timeCost[nodeUPrevIndex][nodeVIndex] + params.timeCost[nodeVIndex][nodeXIndex] - params.timeCost[nodeUPrevIndex][nodeUIndex] - params.timeCost[nodeUIndex][nodeXIndex];
   222		double costSuppV = params.timeCost[nodeVPrevIndex][nodeUIndex] + params.timeCost[nodeUIndex][nodeYIndex] - params.timeCost[nodeVPrevIndex][nodeVIndex] - params.timeCost[nodeVIndex][nodeYIndex];
   223	
   224		if (!intraRouteMove)
   225		{
   226			// Early move pruning to save CPU time. Guarantees that this move cannot improve without checking additional (load, duration...) constraints
   227			if (costSuppU + costSuppV >= routeU->penalty + routeV->penalty) return false;
   228	
   229			costSuppU += penaltyExcessDuration(routeU->duration + costSuppU + serviceV - serviceU)
   230				+ penaltyExcessLoad(routeU->load + loadV - loadU)
   231				- routeU->penalty;
   232	
   233			costSuppV += penaltyExcessDuration(routeV->duration + costSuppV - serviceV + serviceU)
   234				+ penaltyExcessLoad(routeV->load + loadU - loadV)
   235				- routeV->penalty;
   236		}
   237	
   238		if (costSuppU + costSuppV > -MY_EPSILON) return false;
   239		if (nodeUIndex == nodeVPrevIndex || nodeUIndex == nodeYIndex) return false;
   240	
   241		swapNode(nodeU, nodeV);
   242		nbMoves++; // Increment move counter before updating route data
   243		searchCompleted = false;
   244		updateRouteData(routeU);
   245		if (!intraRouteMove) updateRouteData(routeV);
   246		return true;
   247	}
   248	
   249	bool LocalSearch::move5()
   250	{

```

</details>
<details>
<summary>Tool Call: Read file `Program/LocalSearch.cpp` (lines 250-500)</summary>

*Status: Completed*

```HGS-CVRP/Program/LocalSearch.cpp
   250	{
   251		double costSuppU = params.timeCost[nodeUPrevIndex][nodeVIndex] + params.timeCost[nodeVIndex][nodeXNextIndex] - params.timeCost[nodeUPrevIndex][nodeUIndex] - params.timeCost[nodeXIndex][nodeXNextIndex];
   252		double costSuppV = params.timeCost[nodeVPrevIndex][nodeUIndex] + params.timeCost[nodeXIndex][nodeYIndex] - params.timeCost[nodeVPrevIndex][nodeVIndex] - params.timeCost[nodeVIndex][nodeYIndex];
   253	
   254		if (!intraRouteMove)
   255		{
   256			// Early move pruning to save CPU time. Guarantees that this move cannot improve without checking additional (load, duration...) constraints
   257			if (costSuppU + costSuppV >= routeU->penalty + routeV->penalty) return false;
   258	
   259			costSuppU += penaltyExcessDuration(routeU->duration + costSuppU - params.timeCost[nodeUIndex][nodeXIndex] + serviceV - serviceU - serviceX)
   260				+ penaltyExcessLoad(routeU->load + loadV - loadU - loadX)
   261				- routeU->penalty;
   262	
   263			costSuppV += penaltyExcessDuration(routeV->duration + costSuppV + params.timeCost[nodeUIndex][nodeXIndex] - serviceV + serviceU + serviceX)
   264				+ penaltyExcessLoad(routeV->load + loadU + loadX - loadV)
   265				- routeV->penalty;
   266		}
   267	
   268		if (costSuppU + costSuppV > -MY_EPSILON) return false;
   269		if (nodeU == nodeV->prev || nodeX == nodeV->prev || nodeU == nodeY || nodeX->isDepot) return false;
   270	
   271		swapNode(nodeU, nodeV);
   272		insertNode(nodeX, nodeU);
   273		nbMoves++; // Increment move counter before updating route data
   274		searchCompleted = false;
   275		updateRouteData(routeU);
   276		if (!intraRouteMove) updateRouteData(routeV);
   277		return true;
   278	}
   279	
   280	bool LocalSearch::move6()
   281	{
   282		double costSuppU = params.timeCost[nodeUPrevIndex][nodeVIndex] + params.timeCost[nodeYIndex][nodeXNextIndex] - params.timeCost[nodeUPrevIndex][nodeUIndex] - params.timeCost[nodeXIndex][nodeXNextIndex];
   283		double costSuppV = params.timeCost[nodeVPrevIndex][nodeUIndex] + params.timeCost[nodeXIndex][nodeYNextIndex] - params.timeCost[nodeVPrevIndex][nodeVIndex] - params.timeCost[nodeYIndex][nodeYNextIndex];
   284	
   285		if (!intraRouteMove)
   286		{
   287			// Early move pruning to save CPU time. Guarantees that this move cannot improve without checking additional (load, duration...) constraints
   288			if (costSuppU + costSuppV >= routeU->penalty + routeV->penalty) return false;
   289	
   290			costSuppU += penaltyExcessDuration(routeU->duration + costSuppU - params.timeCost[nodeUIndex][nodeXIndex] + params.timeCost[nodeVIndex][nodeYIndex] + serviceV + serviceY - serviceU - serviceX)
   291				+ penaltyExcessLoad(routeU->load + loadV + loadY - loadU - loadX)
   292				- routeU->penalty;
   293	
   294			costSuppV += penaltyExcessDuration(routeV->duration + costSuppV + params.timeCost[nodeUIndex][nodeXIndex] - params.timeCost[nodeVIndex][nodeYIndex] - serviceV - serviceY + serviceU + serviceX)
   295				+ penaltyExcessLoad(routeV->load + loadU + loadX - loadV - loadY)
   296				- routeV->penalty;
   297		}
   298	
   299		if (costSuppU + costSuppV > -MY_EPSILON) return false;
   300		if (nodeX->isDepot || nodeY->isDepot || nodeY == nodeU->prev || nodeU == nodeY || nodeX == nodeV || nodeV == nodeX->next) return false;
   301	
   302		swapNode(nodeU, nodeV);
   303		swapNode(nodeX, nodeY);
   304		nbMoves++; // Increment move counter before updating route data
   305		searchCompleted = false;
   306		updateRouteData(routeU);
   307		if (!intraRouteMove) updateRouteData(routeV);
   308		return true;
   309	}
   310	
   311	bool LocalSearch::move7()
   312	{
   313		if (nodeU->position > nodeV->position) return false;
   314	
   315		double cost = params.timeCost[nodeUIndex][nodeVIndex] + params.timeCost[nodeXIndex][nodeYIndex] - params.timeCost[nodeUIndex][nodeXIndex] - params.timeCost[nodeVIndex][nodeYIndex] + nodeV->cumulatedReversalDistance - nodeX->cumulatedReversalDistance;
   316	
   317		if (cost > -MY_EPSILON) return false;
   318		if (nodeU->next == nodeV) return false;
   319	
   320		Node * nodeNum = nodeX->next;
   321		nodeX->prev = nodeNum;
   322		nodeX->next = nodeY;
   323	
   324		while (nodeNum != nodeV)
   325		{
   326			Node * temp = nodeNum->next;
   327			nodeNum->next = nodeNum->prev;
   328			nodeNum->prev = temp;
   329			nodeNum = temp;
   330		}
   331	
   332		nodeV->next = nodeV->prev;
   333		nodeV->prev = nodeU;
   334		nodeU->next = nodeV;
   335		nodeY->prev = nodeX;
   336	
   337		nbMoves++; // Increment move counter before updating route data
   338		searchCompleted = false;
   339		updateRouteData(routeU);
   340		return true;
   341	}
   342	
   343	bool LocalSearch::move8()
   344	{
   345		double cost = params.timeCost[nodeUIndex][nodeVIndex] + params.timeCost[nodeXIndex][nodeYIndex] - params.timeCost[nodeUIndex][nodeXIndex] - params.timeCost[nodeVIndex][nodeYIndex]
   346			+ nodeV->cumulatedReversalDistance + routeU->reversalDistance - nodeX->cumulatedReversalDistance
   347			- routeU->penalty - routeV->penalty;
   348	
   349		// Early move pruning to save CPU time. Guarantees that this move cannot improve without checking additional (load, duration...) constraints
   350		if (cost >= 0) return false;
   351			
   352		cost += penaltyExcessDuration(nodeU->cumulatedTime + nodeV->cumulatedTime + nodeV->cumulatedReversalDistance + params.timeCost[nodeUIndex][nodeVIndex])
   353			+ penaltyExcessDuration(routeU->duration - nodeU->cumulatedTime - params.timeCost[nodeUIndex][nodeXIndex] + routeU->reversalDistance - nodeX->cumulatedReversalDistance + routeV->duration - nodeV->cumulatedTime - params.timeCost[nodeVIndex][nodeYIndex] + params.timeCost[nodeXIndex][nodeYIndex])
   354			+ penaltyExcessLoad(nodeU->cumulatedLoad + nodeV->cumulatedLoad)
   355			+ penaltyExcessLoad(routeU->load + routeV->load - nodeU->cumulatedLoad - nodeV->cumulatedLoad);
   356			
   357		if (cost > -MY_EPSILON) return false;
   358	
   359		Node * depotU = routeU->depot;
   360		Node * depotV = routeV->depot;
   361		Node * depotUFin = routeU->depot->prev;
   362		Node * depotVFin = routeV->depot->prev;
   363		Node * depotVSuiv = depotV->next;
   364	
   365		Node * temp;
   366		Node * xx = nodeX;
   367		Node * vv = nodeV;
   368	
   369		while (!xx->isDepot)
   370		{
   371			temp = xx->next;
   372			xx->next = xx->prev;
   373			xx->prev = temp;
   374			xx->route = routeV;
   375			xx = temp;
   376		}
   377	
   378		while (!vv->isDepot)
   379		{
   380			temp = vv->prev;
   381			vv->prev = vv->next;
   382			vv->next = temp;
   383			vv->route = routeU;
   384			vv = temp;
   385		}
   386	
   387		nodeU->next = nodeV;
   388		nodeV->prev = nodeU;
   389		nodeX->next = nodeY;
   390		nodeY->prev = nodeX;
   391	
   392		if (nodeX->isDepot)
   393		{
   394			depotUFin->next = depotU;
   395			depotUFin->prev = depotVSuiv;
   396			depotUFin->prev->next = depotUFin;
   397			depotV->next = nodeY;
   398			nodeY->prev = depotV;
   399		}
   400		else if (nodeV->isDepot)
   401		{
   402			depotV->next = depotUFin->prev;
   403			depotV->next->prev = depotV;
   404			depotV->prev = depotVFin;
   405			depotUFin->prev = nodeU;
   406			nodeU->next = depotUFin;
   407		}
   408		else
   409		{
   410			depotV->next = depotUFin->prev;
   411			depotV->next->prev = depotV;
   412			depotUFin->prev = depotVSuiv;
   413			depotUFin->prev->next = depotUFin;
   414		}
   415	
   416		nbMoves++; // Increment move counter before updating route data
   417		searchCompleted = false;
   418		updateRouteData(routeU);
   419		updateRouteData(routeV);
   420		return true;
   421	}
   422	
   423	bool LocalSearch::move9()
   424	{
   425		double cost = params.timeCost[nodeUIndex][nodeYIndex] + params.timeCost[nodeVIndex][nodeXIndex] - params.timeCost[nodeUIndex][nodeXIndex] - params.timeCost[nodeVIndex][nodeYIndex]
   426			        - routeU->penalty - routeV->penalty;
   427	
   428		// Early move pruning to save CPU time. Guarantees that this move cannot improve without checking additional (load, duration...) constraints
   429		if (cost >= 0) return false;
   430			
   431		cost += penaltyExcessDuration(nodeU->cumulatedTime + routeV->duration - nodeV->cumulatedTime - params.timeCost[nodeVIndex][nodeYIndex] + params.timeCost[nodeUIndex][nodeYIndex])
   432			+ penaltyExcessDuration(routeU->duration - nodeU->cumulatedTime - params.timeCost[nodeUIndex][nodeXIndex] + nodeV->cumulatedTime + params.timeCost[nodeVIndex][nodeXIndex])
   433			+ penaltyExcessLoad(nodeU->cumulatedLoad + routeV->load - nodeV->cumulatedLoad)
   434			+ penaltyExcessLoad(nodeV->cumulatedLoad + routeU->load - nodeU->cumulatedLoad);
   435	
   436		if (cost > -MY_EPSILON) return false;
   437	
   438		Node * depotU = routeU->depot;
   439		Node * depotV = routeV->depot;
   440		Node * depotUFin = depotU->prev;
   441		Node * depotVFin = depotV->prev;
   442		Node * depotUpred = depotUFin->prev;
   443	
   444		Node * count = nodeY;
   445		while (!count->isDepot)
   446		{
   447			count->route = routeU;
   448			count = count->next;
   449		}
   450	
   451		count = nodeX;
   452		while (!count->isDepot)
   453		{
   454			count->route = routeV;
   455			count = count->next;
   456		}
   457	
   458		nodeU->next = nodeY;
   459		nodeY->prev = nodeU;
   460		nodeV->next = nodeX;
   461		nodeX->prev = nodeV;
   462	
   463		if (nodeX->isDepot)
   464		{
   465			depotUFin->prev = depotVFin->prev;
   466			depotUFin->prev->next = depotUFin;
   467			nodeV->next = depotVFin;
   468			depotVFin->prev = nodeV;
   469		}
   470		else
   471		{
   472			depotUFin->prev = depotVFin->prev;
   473			depotUFin->prev->next = depotUFin;
   474			depotVFin->prev = depotUpred;
   475			depotVFin->prev->next = depotVFin;
   476		}
   477	
   478		nbMoves++; // Increment move counter before updating route data
   479		searchCompleted = false;
   480		updateRouteData(routeU);
   481		updateRouteData(routeV);
   482		return true;
   483	}
   484	
   485	bool LocalSearch::swapStar()
   486	{
   487		SwapStarElement myBestSwapStar;
   488	
   489		// Preprocessing insertion costs
   490		preprocessInsertions(routeU, routeV);
   491		preprocessInsertions(routeV, routeU);
   492	
   493		// Evaluating the moves
   494		for (nodeU = routeU->depot->next; !nodeU->isDepot; nodeU = nodeU->next)
   495		{
   496			for (nodeV = routeV->depot->next; !nodeV->isDepot; nodeV = nodeV->next)
   497			{
   498				double deltaPenRouteU = penaltyExcessLoad(routeU->load + params.cli[nodeV->cour].demand - params.cli[nodeU->cour].demand) - routeU->penalty;
   499				double deltaPenRouteV = penaltyExcessLoad(routeV->load + params.cli[nodeU->cour].demand - params.cli[nodeV->cour].demand) - routeV->penalty;
   500	

```

</details>
<details>
<summary>Tool Call: Read file `Program/LocalSearch.cpp` (lines 500-812)</summary>

*Status: Completed*

```HGS-CVRP/Program/LocalSearch.cpp
   500	
   501				// Quick filter: possibly early elimination of many SWAP* due to the capacity constraints/penalties and bounds on insertion costs
   502				if (deltaPenRouteU + nodeU->deltaRemoval + deltaPenRouteV + nodeV->deltaRemoval <= 0)
   503				{
   504					SwapStarElement mySwapStar;
   505					mySwapStar.U = nodeU;
   506					mySwapStar.V = nodeV;
   507	
   508					// Evaluate best reinsertion cost of U in the route of V where V has been removed
   509					double extraV = getCheapestInsertSimultRemoval(nodeU, nodeV, mySwapStar.bestPositionU);
   510	
   511					// Evaluate best reinsertion cost of V in the route of U where U has been removed
   512					double extraU = getCheapestInsertSimultRemoval(nodeV, nodeU, mySwapStar.bestPositionV);
   513	
   514					// Evaluating final cost
   515					mySwapStar.moveCost = deltaPenRouteU + nodeU->deltaRemoval + extraU + deltaPenRouteV + nodeV->deltaRemoval + extraV
   516						+ penaltyExcessDuration(routeU->duration + nodeU->deltaRemoval + extraU + params.cli[nodeV->cour].serviceDuration - params.cli[nodeU->cour].serviceDuration)
   517						+ penaltyExcessDuration(routeV->duration + nodeV->deltaRemoval + extraV - params.cli[nodeV->cour].serviceDuration + params.cli[nodeU->cour].serviceDuration);
   518	
   519					if (mySwapStar.moveCost < myBestSwapStar.moveCost)
   520						myBestSwapStar = mySwapStar;
   521				}
   522			}
   523		}
   524	
   525		// Including RELOCATE from nodeU towards routeV (costs nothing to include in the evaluation at this step since we already have the best insertion location)
   526		// Moreover, since the granularity criterion is different, this can lead to different improving moves
   527		for (nodeU = routeU->depot->next; !nodeU->isDepot; nodeU = nodeU->next)
   528		{
   529			SwapStarElement mySwapStar;
   530			mySwapStar.U = nodeU;
   531			mySwapStar.bestPositionU = bestInsertClient[routeV->cour][nodeU->cour].bestLocation[0];
   532			double deltaDistRouteU = params.timeCost[nodeU->prev->cour][nodeU->next->cour] - params.timeCost[nodeU->prev->cour][nodeU->cour] - params.timeCost[nodeU->cour][nodeU->next->cour];
   533			double deltaDistRouteV = bestInsertClient[routeV->cour][nodeU->cour].bestCost[0];
   534			mySwapStar.moveCost = deltaDistRouteU + deltaDistRouteV
   535				+ penaltyExcessLoad(routeU->load - params.cli[nodeU->cour].demand) - routeU->penalty
   536				+ penaltyExcessLoad(routeV->load + params.cli[nodeU->cour].demand) - routeV->penalty
   537				+ penaltyExcessDuration(routeU->duration + deltaDistRouteU - params.cli[nodeU->cour].serviceDuration)
   538				+ penaltyExcessDuration(routeV->duration + deltaDistRouteV + params.cli[nodeU->cour].serviceDuration);
   539	
   540			if (mySwapStar.moveCost < myBestSwapStar.moveCost)
   541				myBestSwapStar = mySwapStar;
   542		}
   543	
   544		// Including RELOCATE from nodeV towards routeU
   545		for (nodeV = routeV->depot->next; !nodeV->isDepot; nodeV = nodeV->next)
   546		{
   547			SwapStarElement mySwapStar;
   548			mySwapStar.V = nodeV;
   549			mySwapStar.bestPositionV = bestInsertClient[routeU->cour][nodeV->cour].bestLocation[0];
   550			double deltaDistRouteU = bestInsertClient[routeU->cour][nodeV->cour].bestCost[0];
   551			double deltaDistRouteV = params.timeCost[nodeV->prev->cour][nodeV->next->cour] - params.timeCost[nodeV->prev->cour][nodeV->cour] - params.timeCost[nodeV->cour][nodeV->next->cour];
   552			mySwapStar.moveCost = deltaDistRouteU + deltaDistRouteV
   553				+ penaltyExcessLoad(routeU->load + params.cli[nodeV->cour].demand) - routeU->penalty
   554				+ penaltyExcessLoad(routeV->load - params.cli[nodeV->cour].demand) - routeV->penalty
   555				+ penaltyExcessDuration(routeU->duration + deltaDistRouteU + params.cli[nodeV->cour].serviceDuration)
   556				+ penaltyExcessDuration(routeV->duration + deltaDistRouteV - params.cli[nodeV->cour].serviceDuration);
   557	
   558			if (mySwapStar.moveCost < myBestSwapStar.moveCost)
   559				myBestSwapStar = mySwapStar;
   560		}
   561	
   562		if (myBestSwapStar.moveCost > -MY_EPSILON) return false;
   563	
   564		// Applying the best move in case of improvement
   565		if (myBestSwapStar.bestPositionU != NULL) insertNode(myBestSwapStar.U, myBestSwapStar.bestPositionU);
   566		if (myBestSwapStar.bestPositionV != NULL) insertNode(myBestSwapStar.V, myBestSwapStar.bestPositionV);
   567		nbMoves++; // Increment move counter before updating route data
   568		searchCompleted = false;
   569		updateRouteData(routeU);
   570		updateRouteData(routeV);
   571		return true;
   572	}
   573	
   574	double LocalSearch::getCheapestInsertSimultRemoval(Node * U, Node * V, Node *& bestPosition)
   575	{
   576		ThreeBestInsert * myBestInsert = &bestInsertClient[V->route->cour][U->cour];
   577		bool found = false;
   578	
   579		// Find best insertion in the route such that V is not next or pred (can only belong to the top three locations)
   580		bestPosition = myBestInsert->bestLocation[0];
   581		double bestCost = myBestInsert->bestCost[0];
   582		found = (bestPosition != V && bestPosition->next != V);
   583		if (!found && myBestInsert->bestLocation[1] != NULL)
   584		{
   585			bestPosition = myBestInsert->bestLocation[1];
   586			bestCost = myBestInsert->bestCost[1];
   587			found = (bestPosition != V && bestPosition->next != V);
   588			if (!found && myBestInsert->bestLocation[2] != NULL)
   589			{
   590				bestPosition = myBestInsert->bestLocation[2];
   591				bestCost = myBestInsert->bestCost[2];
   592				found = true;
   593			}
   594		}
   595	
   596		// Compute insertion in the place of V
   597		double deltaCost = params.timeCost[V->prev->cour][U->cour] + params.timeCost[U->cour][V->next->cour] - params.timeCost[V->prev->cour][V->next->cour];
   598		if (!found || deltaCost < bestCost)
   599		{
   600			bestPosition = V->prev;
   601			bestCost = deltaCost;
   602		}
   603	
   604		return bestCost;
   605	}
   606	
   607	void LocalSearch::preprocessInsertions(Route * R1, Route * R2)
   608	{
   609		for (Node * U = R1->depot->next; !U->isDepot; U = U->next)
   610		{
   611			// Performs the preprocessing
   612			U->deltaRemoval = params.timeCost[U->prev->cour][U->next->cour] - params.timeCost[U->prev->cour][U->cour] - params.timeCost[U->cour][U->next->cour];
   613			if (R2->whenLastModified > bestInsertClient[R2->cour][U->cour].whenLastCalculated)
   614			{
   615				bestInsertClient[R2->cour][U->cour].reset();
   616				bestInsertClient[R2->cour][U->cour].whenLastCalculated = nbMoves;
   617				bestInsertClient[R2->cour][U->cour].bestCost[0] = params.timeCost[0][U->cour] + params.timeCost[U->cour][R2->depot->next->cour] - params.timeCost[0][R2->depot->next->cour];
   618				bestInsertClient[R2->cour][U->cour].bestLocation[0] = R2->depot;
   619				for (Node * V = R2->depot->next; !V->isDepot; V = V->next)
   620				{
   621					double deltaCost = params.timeCost[V->cour][U->cour] + params.timeCost[U->cour][V->next->cour] - params.timeCost[V->cour][V->next->cour];
   622					bestInsertClient[R2->cour][U->cour].compareAndAdd(deltaCost, V);
   623				}
   624			}
   625		}
   626	}
   627	
   628	void LocalSearch::insertNode(Node * U, Node * V)
   629	{
   630		U->prev->next = U->next;
   631		U->next->prev = U->prev;
   632		V->next->prev = U;
   633		U->prev = V;
   634		U->next = V->next;
   635		V->next = U;
   636		U->route = V->route;
   637	}
   638	
   639	void LocalSearch::swapNode(Node * U, Node * V)
   640	{
   641		Node * myVPred = V->prev;
   642		Node * myVSuiv = V->next;
   643		Node * myUPred = U->prev;
   644		Node * myUSuiv = U->next;
   645		Route * myRouteU = U->route;
   646		Route * myRouteV = V->route;
   647	
   648		myUPred->next = V;
   649		myUSuiv->prev = V;
   650		myVPred->next = U;
   651		myVSuiv->prev = U;
   652	
   653		U->prev = myVPred;
   654		U->next = myVSuiv;
   655		V->prev = myUPred;
   656		V->next = myUSuiv;
   657	
   658		U->route = myRouteV;
   659		V->route = myRouteU;
   660	}
   661	
   662	void LocalSearch::updateRouteData(Route * myRoute)
   663	{
   664		int myplace = 0;
   665		double myload = 0.;
   666		double mytime = 0.;
   667		double myReversalDistance = 0.;
   668		double cumulatedX = 0.;
   669		double cumulatedY = 0.;
   670	
   671		Node * mynode = myRoute->depot;
   672		mynode->position = 0;
   673		mynode->cumulatedLoad = 0.;
   674		mynode->cumulatedTime = 0.;
   675		mynode->cumulatedReversalDistance = 0.;
   676	
   677		bool firstIt = true;
   678		while (!mynode->isDepot || firstIt)
   679		{
   680			mynode = mynode->next;
   681			myplace++;
   682			mynode->position = myplace;
   683			myload += params.cli[mynode->cour].demand;
   684			mytime += params.timeCost[mynode->prev->cour][mynode->cour] + params.cli[mynode->cour].serviceDuration;
   685			myReversalDistance += params.timeCost[mynode->cour][mynode->prev->cour] - params.timeCost[mynode->prev->cour][mynode->cour] ;
   686			mynode->cumulatedLoad = myload;
   687			mynode->cumulatedTime = mytime;
   688			mynode->cumulatedReversalDistance = myReversalDistance;
   689			if (!mynode->isDepot)
   690			{
   691				cumulatedX += params.cli[mynode->cour].coordX;
   692				cumulatedY += params.cli[mynode->cour].coordY;
   693				if (firstIt) myRoute->sector.initialize(params.cli[mynode->cour].polarAngle);
   694				else myRoute->sector.extend(params.cli[mynode->cour].polarAngle);
   695			}
   696			firstIt = false;
   697		}
   698	
   699		myRoute->duration = mytime;
   700		myRoute->load = myload;
   701		myRoute->penalty = penaltyExcessDuration(mytime) + penaltyExcessLoad(myload);
   702		myRoute->nbCustomers = myplace-1;
   703		myRoute->reversalDistance = myReversalDistance;
   704		// Remember "when" this route has been last modified (will be used to filter unnecessary move evaluations)
   705		myRoute->whenLastModified = nbMoves ;
   706	
   707		if (myRoute->nbCustomers == 0)
   708		{
   709			myRoute->polarAngleBarycenter = 1.e30;
   710			emptyRoutes.insert(myRoute->cour);
   711		}
   712		else
   713		{
   714			myRoute->polarAngleBarycenter = atan2(cumulatedY/(double)myRoute->nbCustomers - params.cli[0].coordY, cumulatedX/(double)myRoute->nbCustomers - params.cli[0].coordX);
   715			emptyRoutes.erase(myRoute->cour);
   716		}
   717	}
   718	
   719	void LocalSearch::loadIndividual(const Individual & indiv)
   720	{
   721		emptyRoutes.clear();
   722		nbMoves = 0; 
   723		for (int r = 0; r < params.nbVehicles; r++)
   724		{
   725			Node * myDepot = &depots[r];
   726			Node * myDepotFin = &depotsEnd[r];
   727			Route * myRoute = &routes[r];
   728			myDepot->prev = myDepotFin;
   729			myDepotFin->next = myDepot;
   730			if (!indiv.chromR[r].empty())
   731			{
   732				Node * myClient = &clients[indiv.chromR[r][0]];
   733				myClient->route = myRoute;
   734				myClient->prev = myDepot;
   735				myDepot->next = myClient;
   736				for (int i = 1; i < (int)indiv.chromR[r].size(); i++)
   737				{
   738					Node * myClientPred = myClient;
   739					myClient = &clients[indiv.chromR[r][i]]; 
   740					myClient->prev = myClientPred;
   741					myClientPred->next = myClient;
   742					myClient->route = myRoute;
   743				}
   744				myClient->next = myDepotFin;
   745				myDepotFin->prev = myClient;
   746			}
   747			else
   748			{
   749				myDepot->next = myDepotFin;
   750				myDepotFin->prev = myDepot;
   751			}
   752			updateRouteData(&routes[r]);
   753			routes[r].whenLastTestedSWAPStar = -1;
   754			for (int i = 1; i <= params.nbClients; i++) // Initializing memory structures
   755				bestInsertClient[r][i].whenLastCalculated = -1;
   756		}
   757	
   758		for (int i = 1; i <= params.nbClients; i++) // Initializing memory structures
   759			clients[i].whenLastTestedRI = -1;
   760	}
   761	
   762	void LocalSearch::exportIndividual(Individual & indiv)
   763	{
   764		std::vector < std::pair <double, int> > routePolarAngles ;
   765		for (int r = 0; r < params.nbVehicles; r++)
   766			routePolarAngles.push_back(std::pair <double, int>(routes[r].polarAngleBarycenter, r));
   767		std::sort(routePolarAngles.begin(), routePolarAngles.end()); // empty routes have a polar angle of 1.e30, and therefore will always appear at the end
   768	
   769		int pos = 0;
   770		for (int r = 0; r < params.nbVehicles; r++)
   771		{
   772			indiv.chromR[r].clear();
   773			Node * node = depots[routePolarAngles[r].second].next;
   774			while (!node->isDepot)
   775			{
   776				indiv.chromT[pos] = node->cour;
   777				indiv.chromR[r].push_back(node->cour);
   778				node = node->next;
   779				pos++;
   780			}
   781		}
   782	
   783		indiv.evaluateCompleteCost(params);
   784	}
   785	
   786	LocalSearch::LocalSearch(Params & params) : params (params)
   787	{
   788		clients = std::vector < Node >(params.nbClients + 1);
   789		routes = std::vector < Route >(params.nbVehicles);
   790		depots = std::vector < Node >(params.nbVehicles);
   791		depotsEnd = std::vector < Node >(params.nbVehicles);
   792		bestInsertClient = std::vector < std::vector <ThreeBestInsert> >(params.nbVehicles, std::vector <ThreeBestInsert>(params.nbClients + 1));
   793	
   794		for (int i = 0; i <= params.nbClients; i++) 
   795		{ 
   796			clients[i].cour = i; 
   797			clients[i].isDepot = false; 
   798		}
   799		for (int i = 0; i < params.nbVehicles; i++)
   800		{
   801			routes[i].cour = i;
   802			routes[i].depot = &depots[i];
   803			depots[i].cour = 0;
   804			depots[i].isDepot = true;
   805			depots[i].route = &routes[i];
   806			depotsEnd[i].cour = 0;
   807			depotsEnd[i].isDepot = true;
   808			depotsEnd[i].route = &routes[i];
   809		}
   810		for (int i = 1 ; i <= params.nbClients ; i++) orderNodes.push_back(i);
   811		for (int r = 0 ; r < params.nbVehicles ; r++) orderRoutes.push_back(r);
   812	}

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

I need to go through the remaining source files to understand the full structure.

</details>

现在阅读剩余的文件:

<details>
<summary>Tool Call: Read file `Program/InstanceCVRPLIB.cpp`</summary>

*Status: Completed*

```HGS-CVRP/Program/InstanceCVRPLIB.cpp
     1	//
     2	// Created by chkwon on 3/22/22.
     3	//
     4	
     5	#include <fstream>
     6	#include <cmath>
     7	#include "InstanceCVRPLIB.h"
     8	
     9	InstanceCVRPLIB::InstanceCVRPLIB(std::string pathToInstance, bool isRoundingInteger = true)
    10	{
    11		std::string content, content2, content3;
    12		double serviceTimeData = 0.;
    13	
    14		// Read INPUT dataset
    15		std::ifstream inputFile(pathToInstance);
    16		if (inputFile.is_open())
    17		{
    18			getline(inputFile, content);
    19			getline(inputFile, content);
    20			getline(inputFile, content);
    21			for (inputFile >> content ; content != "NODE_COORD_SECTION" ; inputFile >> content)
    22			{
    23				if (content == "DIMENSION") { inputFile >> content2 >> nbClients; nbClients--; } // Need to substract the depot from the number of nodes
    24				else if (content == "EDGE_WEIGHT_TYPE")	inputFile >> content2 >> content3;
    25				else if (content == "CAPACITY")	inputFile >> content2 >> vehicleCapacity;
    26				else if (content == "DISTANCE") { inputFile >> content2 >> durationLimit; isDurationConstraint = true; }
    27				else if (content == "SERVICE_TIME")	inputFile >> content2 >> serviceTimeData;
    28				else throw std::string("Unexpected data in input file: " + content);
    29			}
    30			if (nbClients <= 0) throw std::string("Number of nodes is undefined");
    31			if (vehicleCapacity == 1.e30) throw std::string("Vehicle capacity is undefined");
    32	
    33			x_coords = std::vector<double>(nbClients + 1);
    34			y_coords = std::vector<double>(nbClients + 1);
    35			demands = std::vector<double>(nbClients + 1);
    36			service_time = std::vector<double>(nbClients + 1);
    37	
    38			// Reading node coordinates
    39			// depot must be the first element
    40			// 		- i = 0 in the for-loop below, or
    41			// 		- node_number = 1 in the .vrp file
    42			// customers are
    43			// 		- i = 1, 2, ..., nbClients in the for-loop below, or
    44			// 		- node_number = 2, 3, ..., nb_Clients in the .vrp file
    45			int node_number;
    46			for (int i = 0; i <= nbClients; i++)
    47			{
    48				inputFile >> node_number >> x_coords[i] >> y_coords[i];
    49				if (node_number != i + 1) throw std::string("The node numbering is not in order.");
    50			}
    51	
    52			// Reading demand information
    53			inputFile >> content;
    54			if (content != "DEMAND_SECTION") throw std::string("Unexpected data in input file: " + content);
    55			for (int i = 0; i <= nbClients; i++)
    56			{
    57				inputFile >> content >> demands[i];
    58				service_time[i] = (i == 0) ? 0. : serviceTimeData ;
    59			}
    60	
    61			// Calculating 2D Euclidean Distance
    62			dist_mtx = std::vector < std::vector< double > >(nbClients + 1, std::vector <double>(nbClients + 1));
    63			for (int i = 0; i <= nbClients; i++)
    64			{
    65				for (int j = 0; j <= nbClients; j++)
    66				{
    67					dist_mtx[i][j] = std::sqrt(
    68						(x_coords[i] - x_coords[j]) * (x_coords[i] - x_coords[j])
    69						+ (y_coords[i] - y_coords[j]) * (y_coords[i] - y_coords[j])
    70					);
    71	
    72					if (isRoundingInteger) dist_mtx[i][j] = round(dist_mtx[i][j]);
    73				}
    74			}
    75	
    76			// Reading depot information (in all current instances the depot is represented as node 1, the program will return an error otherwise)
    77			inputFile >> content >> content2 >> content3 >> content3;
    78			if (content != "DEPOT_SECTION") throw std::string("Unexpected data in input file: " + content);
    79			if (content2 != "1") throw std::string("Expected depot index 1 instead of " + content2);
    80			if (content3 != "EOF") throw std::string("Unexpected data in input file: " + content3);
    81		}
    82		else
    83			throw std::string("Impossible to open instance file: " + pathToInstance);
    84	}

```

</details>
<details>
<summary>Tool Call: Read file `Program/InstanceCVRPLIB.h`</summary>

*Status: Completed*

```HGS-CVRP/Program/InstanceCVRPLIB.h
     1	//
     2	// Created by chkwon on 3/22/22.
     3	//
     4	
     5	#ifndef INSTANCECVRPLIB_H
     6	#define INSTANCECVRPLIB_H
     7	#include<string>
     8	#include<vector>
     9	
    10	class InstanceCVRPLIB
    11	{
    12	public:
    13		std::vector<double> x_coords;
    14		std::vector<double> y_coords;
    15		std::vector< std::vector<double> > dist_mtx;
    16		std::vector<double> service_time;
    17		std::vector<double> demands;
    18		double durationLimit = 1.e30;							// Route duration limit
    19		double vehicleCapacity = 1.e30;							// Capacity limit
    20		bool isDurationConstraint = false;						// Indicates if the problem includes duration constraints
    21		int nbClients ;											// Number of clients (excluding the depot)
    22	
    23		InstanceCVRPLIB(std::string pathToInstance, bool isRoundingInteger);
    24	};
    25	
    26	
    27	#endif //INSTANCECVRPLIB_H

```

</details>
<details>
<summary>Tool Call: Read file `Program/commandline.h`</summary>

*Status: Completed*

```HGS-CVRP/Program/commandline.h
     1	/*MIT License
     2	
     3	Copyright(c) 2020 Thibaut Vidal
     4	
     5	Permission is hereby granted, free of charge, to any person obtaining a copy
     6	of this software and associated documentation files(the "Software"), to deal
     7	in the Software without restriction, including without limitation the rights
     8	to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
     9	copies of the Software, and to permit persons to whom the Software is
    10	furnished to do so, subject to the following conditions :
    11	
    12	The above copyright notice and this permission notice shall be included in all
    13	copies or substantial portions of the Software.
    14	
    15	THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    16	IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    17	FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE
    18	AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    19	LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    20	OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    21	SOFTWARE.*/
    22	
    23	#ifndef COMMAND_LINE_H
    24	#define COMMAND_LINE_H
    25	
    26	#include <iostream>
    27	#include <string>
    28	#include <climits>
    29	#include "AlgorithmParameters.h"
    30	
    31	class CommandLine
    32	{
    33	public:
    34		AlgorithmParameters ap = default_algorithm_parameters();
    35	
    36		int nbVeh		 = INT_MAX;		// Number of vehicles. Default value: infinity
    37		std::string pathInstance;		// Instance path
    38		std::string pathSolution;		// Solution path
    39		bool verbose     = true;
    40		bool isRoundingInteger = true;
    41	
    42		// Reads the line of command and extracts possible options
    43		CommandLine(int argc, char* argv[])
    44		{
    45			if (argc % 2 != 1 || argc > 35 || argc < 3)
    46			{
    47				std::cout << "----- NUMBER OF COMMANDLINE ARGUMENTS IS INCORRECT: " << argc << std::endl;
    48				display_help(); throw std::string("Incorrect line of command");
    49			}
    50			else
    51			{
    52				pathInstance = std::string(argv[1]);
    53				pathSolution = std::string(argv[2]);
    54				for (int i = 3; i < argc; i += 2)
    55				{
    56					if (std::string(argv[i]) == "-t")
    57						ap.timeLimit = atof(argv[i+1]);
    58					else if (std::string(argv[i]) == "-it")
    59						ap.nbIter  = atoi(argv[i+1]);
    60					else if (std::string(argv[i]) == "-seed")
    61						ap.seed    = atoi(argv[i+1]);
    62					else if (std::string(argv[i]) == "-veh")
    63						nbVeh = atoi(argv[i+1]);
    64					else if (std::string(argv[i]) == "-round")
    65						isRoundingInteger = atoi(argv[i+1]);
    66					else if (std::string(argv[i]) == "-log")
    67						verbose = atoi(argv[i+1]);
    68					else if (std::string(argv[i]) == "-nbGranular")
    69						ap.nbGranular = atoi(argv[i+1]);
    70					else if (std::string(argv[i]) == "-mu")
    71						ap.mu = atoi(argv[i+1]);
    72					else if (std::string(argv[i]) == "-lambda")
    73						ap.lambda = atoi(argv[i+1]);
    74					else if (std::string(argv[i]) == "-nbElite")
    75						ap.nbElite = atoi(argv[i+1]);
    76					else if (std::string(argv[i]) == "-nbClose")
    77						ap.nbClose = atoi(argv[i+1]);
    78					else if (std::string(argv[i]) == "-nbIterPenaltyManagement")
    79						ap.nbIterPenaltyManagement = atoi(argv[i+1]);
    80					else if (std::string(argv[i]) == "-nbIterTraces")
    81						ap.nbIterTraces = atoi(argv[i + 1]);
    82					else if (std::string(argv[i]) == "-targetFeasible")
    83						ap.targetFeasible = atof(argv[i+1]);
    84					else if (std::string(argv[i]) == "-penaltyIncrease")
    85						ap.penaltyIncrease = atof(argv[i+1]);
    86					else if (std::string(argv[i]) == "-penaltyDecrease")
    87						ap.penaltyDecrease = atof(argv[i+1]);
    88					else
    89					{
    90						std::cout << "----- ARGUMENT NOT RECOGNIZED: " << std::string(argv[i]) << std::endl;
    91						display_help(); throw std::string("Incorrect line of command");
    92					}
    93				}
    94			}
    95		}
    96	
    97		// Printing information about how to use the code
    98		void display_help()
    99		{
   100			std::cout << std::endl;
   101			std::cout << "-------------------------------------------------- HGS-CVRP algorithm (2020) ---------------------------------------------------" << std::endl;
   102			std::cout << "Call with: ./hgs instancePath solPath [-it nbIter] [-t myCPUtime] [-seed mySeed] [-veh nbVehicles] [-log verbose]               " << std::endl;
   103			std::cout << "[-it <int>] sets a maximum number of iterations without improvement. Defaults to 20,000                                         " << std::endl;
   104			std::cout << "[-t <double>] sets a time limit in seconds. If this parameter is set the code will be run iteratively until the time limit      " << std::endl;
   105			std::cout << "[-seed <int>] sets a fixed seed. Defaults to 0                                                                                  " << std::endl;
   106			std::cout << "[-veh <int>] sets a prescribed fleet size. Otherwise a reasonable UB on the the fleet size is calculated                        " << std::endl;
   107			std::cout << "[-round <bool>] rounding the distance to the nearest integer or not. It can be 0 (not rounding) or 1 (rounding). Defaults to 1. " << std::endl;
   108			std::cout << "[-log <bool>] sets the verbose level of the algorithm log. It can be 0 or 1. Defaults to 1.                                     " << std::endl;
   109			std::cout << std::endl;
   110			std::cout << "Additional Arguments:                                                                                                           " << std::endl;
   111			std::cout << "[-nbIterTraces <int>] Number of iterations between traces display during HGS execution. Defaults to 500                         " << std::endl;
   112			std::cout << "[-nbGranular <int>] Granular search parameter, limits the number of moves in the RI local search. Defaults to 20                " << std::endl;
   113			std::cout << "[-mu <int>] Minimum population size. Defaults to 25                                                                             " << std::endl;
   114			std::cout << "[-lambda <int>] Number of solutions created before reaching the maximum population size (i.e., generation size). Defaults to 40 " << std::endl;
   115			std::cout << "[-nbElite <int>] Number of elite individuals. Defaults to 5                                                                     " << std::endl;
   116			std::cout << "[-nbClose <int>] Number of closest solutions/individuals considered when calculating diversity contribution. Defaults to 4      " << std::endl;
   117			std::cout << "[-nbIterPenaltyManagement <int>] Number of iterations between penalty updates. Defaults to 100                                  " << std::endl;
   118			std::cout << "[-targetFeasible <double>] target ratio of feasible individuals between penalty updates. Defaults to 0.2                        " << std::endl;
   119			std::cout << "[-penaltyIncrease <double>] penalty increase if insufficient feasible individuals between penalty updates. Defaults to 1.2      " << std::endl;
   120			std::cout << "[-penaltyDecrease <double>] penalty decrease if sufficient feasible individuals between penalty updates. Defaults to 0.85       " << std::endl;
   121			std::cout << "--------------------------------------------------------------------------------------------------------------------------------" << std::endl;
   122			std::cout << std::endl;
   123		};
   124	};
   125	#endif

```

</details>
<details>
<summary>Tool Call: Read file `Program/main.cpp`</summary>

*Status: Completed*

```HGS-CVRP/Program/main.cpp
     1	#include "Genetic.h"
     2	#include "commandline.h"
     3	#include "LocalSearch.h"
     4	#include "Split.h"
     5	#include "InstanceCVRPLIB.h"
     6	using namespace std;
     7	
     8	int main(int argc, char *argv[])
     9	{
    10		try
    11		{
    12			// Reading the arguments of the program
    13			CommandLine commandline(argc, argv);
    14	
    15			// Print all algorithm parameter values
    16			if (commandline.verbose) print_algorithm_parameters(commandline.ap);
    17	
    18			// Reading the data file and initializing some data structures
    19			if (commandline.verbose) std::cout << "----- READING INSTANCE: " << commandline.pathInstance << std::endl;
    20			InstanceCVRPLIB cvrp(commandline.pathInstance, commandline.isRoundingInteger);
    21	
    22			Params params(cvrp.x_coords,cvrp.y_coords,cvrp.dist_mtx,cvrp.service_time,cvrp.demands,
    23				          cvrp.vehicleCapacity,cvrp.durationLimit,commandline.nbVeh,cvrp.isDurationConstraint,commandline.verbose,commandline.ap);
    24	
    25			// Running HGS
    26			Genetic solver(params);
    27			solver.run();
    28			
    29			// Exporting the best solution
    30			if (solver.population.getBestFound() != NULL)
    31			{
    32				if (params.verbose) std::cout << "----- WRITING BEST SOLUTION IN : " << commandline.pathSolution << std::endl;
    33				solver.population.exportCVRPLibFormat(*solver.population.getBestFound(),commandline.pathSolution);
    34				solver.population.exportSearchProgress(commandline.pathSolution + ".PG.csv", commandline.pathInstance);
    35			}
    36		}
    37		catch (const string& e) { std::cout << "EXCEPTION | " << e << std::endl; }
    38		catch (const std::exception& e) { std::cout << "EXCEPTION | " << e.what() << std::endl; }
    39		return 0;
    40	}

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

I'm getting a complete view of the C++ codebase structure, and now I want to examine the C_Interface and Test folders along with the CMakeLists to see what the full testing setup looks like.让我先查看测试目录和 C 接口来了解完整的项目范围。

</details>

<details>
<summary>Tool Call: List the `HGS-CVRP/Test` directory's contents</summary>

*Status: Completed*

# Folders:
HGS-CVRP/Test/Test-c

# Files:
HGS-CVRP/Test/TestExecutable.cmake

</details>
<details>
<summary>Tool Call: Read file `Program/C_Interface.h`</summary>

*Status: Completed*

```HGS-CVRP/Program/C_Interface.h
     1	//
     2	// Created by chkwon on 3/23/22.
     3	//
     4	
     5	#ifndef C_INTERFACE_H
     6	#define C_INTERFACE_H
     7	#include "AlgorithmParameters.h"
     8	
     9	struct SolutionRoute
    10	{
    11		int length;
    12		int * path;
    13	};
    14	
    15	struct Solution
    16	{
    17		double cost;
    18		double time;
    19		int n_routes;
    20		struct SolutionRoute * routes;
    21	};
    22	
    23	#ifdef __cplusplus
    24	extern "C"
    25	#endif
    26	struct Solution * solve_cvrp(
    27		int n, double* x, double* y, double* serv_time, double* dem,
    28		double vehicleCapacity, double durationLimit, char isRoundingInteger, char isDurationConstraint,
    29		int max_nbVeh, const struct AlgorithmParameters* ap, char verbose);
    30	
    31	#ifdef __cplusplus
    32	extern "C"
    33	#endif
    34	struct Solution *solve_cvrp_dist_mtx(
    35		int n, double* x, double* y, double *dist_mtx, double *serv_time, double *dem,
    36		double vehicleCapacity, double durationLimit, char isDurationConstraint,
    37		int max_nbVeh, const struct AlgorithmParameters *ap, char verbose);
    38	
    39	#ifdef __cplusplus
    40	extern "C"
    41	#endif
    42	void delete_solution(struct Solution * sol);
    43	
    44	
    45	#endif //C_INTERFACE_H

```

</details>
<details>
<summary>Tool Call: List the `HGS-CVRP/Instances` directory's contents</summary>

*Status: Completed*

# Folders:
HGS-CVRP/Instances/CVRP

</details>
## Assistant

<details>
<summary>思考过程</summary>

I'm looking at the instances folder and examining how the C++ code generates random numbers using `std::minstd_rand`, which is a linear congruential generator with the parameters 48271, 0, and 2147483647. The formula is straightforward to replicate in Rust: multiply by 48271 and take modulo 2147483647. I'm also noting that `std::shuffle` and `std::uniform_int_distribution` have implementation-defined behavior, which I'll need to account for.

The user's requirement allows for differences due to language-specific factors like RNG implementations, so I can implement `minstd_rand` exactly as specified by the standard (with seed handling where 0 becomes 1), and then use a reasonable Fisher-Yates shuffle with uniform distribution on top. Now I'm starting to sketch out the Rust project structure. For the data structures, I'm thinking through how to handle the linked lists and population management without raw pointers. The key insight is to use index-based approaches: storing nodes and routes in vectors with usize indices for next/prev pointers instead of actual pointers. For the population, I could use a slab-based arena where individuals are stored in a Vec with optional slots and a free-list for reuse, with proximity relationships tracked as sorted vectors of (distance, individual_id) pairs rather than C++'s multiset of pointers.

I'm realizing that when distances are equal in the proximity list, the tie-breaking order doesn't actually affect the sum of k smallest distances—any ordering of tied values produces the same result. For removing individuals, I just need to find and delete the entry with the matching id from the proximity list. For LocalSearch, I'll consolidate all nodes (clients, depots, depot ends) into a single Vec<Node> arena indexed by usize, where the layout is clients at 0..=n, then depots and depot ends follow, making next/prev navigation straightforward with simple index arithmetic.

I'm defining the Node and Route structures with fields for tracking position, load, time, reversal distance, and other cached values. The timeCost matrix will be a flat Vec<f64> for better cache locality. For the RNG, I'm implementing minstd_rand with the standard linear congruential formula, ensuring the seed is in the valid range [1, 2147483646], and then I need to handle the uniform_int_distribution scaling to match libstdc++'s behavior. Implementing the libstdc++ shuffle algorithm... I'll implement the full libstdc++ shuffle algorithm including the paired integer generation to match the reference behavior closely, then handle the direct RNG calls for modulo operations. For timing, I'll use wall-clock time with `std::time::Instant` since it's more portable than CPU time, and I'll document this choice in the README.

For error handling, I'll define a simple custom `Result` type using `String` errors to keep dependencies minimal. On the population side, when adding individuals to the subpopulation, I need to clone them since both the population and the offspring buffer need to retain ownership of the data—this is a semantic requirement, not an unnecessary copy. I could alternatively generate fresh individuals each iteration and move them in, but that would allocate new chromosome buffers repeatedly, which is less efficient than C++'s approach of reusing the offspring buffer.

Looking at the `Individual` structure, it contains `indivsPerProximity` which gets copied along with the individual, but this multiset stays empty in the offspring itself—only the copies stored in the population get their proximity data populated during the genetic algorithm loop. So the copy operation is clean and doesn't carry unnecessary baggage.

For the Rust implementation, I'm separating concerns by storing proximity and biased fitness in a wrapper `PopIndividual` struct rather than embedding them in `Individual` itself—this is cleaner than the C++ design and avoids the coupling the original code even comments about. For the proximity multiset, I can maintain a sorted vector by distance alone (using binary search insertion), which is deterministic and behavior-equivalent since tie-breaking order doesn't affect the distance sums or clone detection checks. The ranking in `updateBiasedFitnesses` sorts pairs of distance and index, so I'll use Rust's standard pair sorting which handles ties consistently.

Now I'm hitting a borrow checker issue: `getBinaryTournament` needs to call `updateBiasedFitnesses` (which mutates the population) and return a reference to an individual from that same population. When crossover needs two parents from successive tournament calls, I can't hold both references simultaneously while also calling `&mut self` for the fitness updates—I need to rethink the API to either return owned copies or restructure the borrowing.

My approach: have `get_binary_tournament` return a stable slot identifier instead of a reference, then fetch both parent references after both tournament calls complete. Since `updateBiasedFitnesses` only updates values and doesn't reorder, the indices stay valid between calls.

The trickier issue is RNG ownership. In C++, `Params` holds the RNG and everything borrows it mutably. In Rust, I need to avoid threading `&mut Params` everywhere—it conflicts with the borrow checker when, say, `LocalSearch` holds `&Params` but needs mutable access to the RNG. I'm considering splitting `Params` into immutable problem data and a separate mutable state for penalties and RNG that gets passed explicitly where needed. The penalties struct gets passed in two forms: one for the local search algorithm itself (which might be scaled differently, like 10x for repair) and one for the final evaluation using global penalties. So the signature becomes `run(&mut self, indiv, penalties_for_ls, global_penalties, rng)` to handle both the scaled penalties during search and the standard penalties for the final cost evaluation.

For the population methods, I need to thread through params (for client count and algorithm settings), clock for timestamps, and handle the mutual borrowing issue in Rust where Population needs to call LocalSearch and Split—that's better solved by moving the generation logic into the Genetic struct itself since it owns all three components.

For state management, I'm deciding whether to keep references to Params in structs (matching the C++ design but requiring lifetime annotations) or pass them per-call (more idiomatic Rust, avoids self-referential issues). The per-call approach seems cleaner here. Genetic will own the mutable pieces like RNG and Penalties, while Params stays immutable and either gets borrowed or lives in Genetic. Penalties is a small Copy struct with capacity and duration fields, owned by Genetic and passed by value throughout. RNG is also owned by Genetic and passed mutably where needed—Population uses it for repair logic, LocalSearch for shuffling, and Individual construction for crossover. For Individual, I'm using usize for client indices since that's idiomatic for Rust indexing, though polarAngle stays as i32 for arithmetic. The penalizedCost starts at a large value like 1e30. Now I'm sketching out the LocalSearch structure with a linked list using indices to track node connections.

Defining the Node struct with fields for depot status, client index, position, last tested time, arena pointers, route assignment, and accumulated metrics like load and time. The arena layout packs client nodes first (indexed by client id), then depot start/end nodes for each route, with helper functions to locate them. Routes track their index, customer count, modification time, swap testing time, depot reference, and metrics including duration, load, reversal distance, and polar angle barycenter. For empty routes, I need a set-like structure that efficiently tracks the smallest index.

Using BTreeSet for empty routes since it supports insertion, removal, emptiness checks, and retrieving the first element. The move methods translate from C++ pointer surgery to index-based operations on the nodes arena, requiring careful borrow checking when reading then writing. For preprocessInsertions, the best insertion candidates are stored as node ids with Option<usize> to handle null cases, where depot insertions are always valid but other positions may be unavailable. The location array will always have a valid value at index 0 because preprocessing fills it on the first call, and the timing logic ensures it's recalculated before use. In the swap operations, bestPositionU can be null when the best move is a pure relocation without U, which maps naturally to Option in Rust. I'm defining the ThreeBestInsert struct with arrays for cost and location, where location uses Option to handle the null case, and this structure gets allocated once per vehicle-client pair.

For updateRouteData, I need to replicate the exact loop behavior including how the end-depot gets its position values set, and handle the empty route case where polarAngleBarycenter returns a sentinel value that gets sorted deterministically by route index. The genetic algorithm's crossover needs careful attention to RNG sequencing—C++ argument evaluation order is unspecified, but I'll use left-to-right evaluation for the two tournament selections to keep the RNG sequence predictable. The crossoverOX function draws two random positions for the segment boundaries...

Continuing with the copy and fill loops...

The array bounds check: potential and pred are sized (nbVehicles+1) × (nbClients+1), and maxVehicles is capped at nbVehicles, so all row accesses stay within bounds. In generalSplit, cliSplit[0].dnext initializes to 0 by default, and sumDistance builds from there—the -1e30 sentinel at the last position never gets used in the distance sum calculation since we only access up to nbClients-1. For the population output, I need to replicate the printf format strings for iteration count, time, feasibility stats, infeasibility stats, diversity, and penalty metrics.

For solution export, the C++ code uses default ostream formatting for the penalized cost, which applies %g-style precision (6 significant digits)—this can switch to scientific notation for large values like 1046781.5 becoming "1.04678e+06". Rust's default f64 Display is more sensible, printing the shortest round-tripping representation, so I'll just use `{}` formatting. Since costs are typically integers after rounding and stay well under 999999 for the benchmark instances, this shouldn't cause issues.

For the PG.csv export, I need to output instanceName, seed, cost, and time in semicolon-separated format. The instance parser needs to replicate the C++ token-based logic: skip the first three header lines, then loop through tokens until hitting "NODE_COORD_SECTION", handling DIMENSION, EDGE_WEIGHT_TYPE, CAPACITY, DISTANCE, and SERVICE_TIME fields with their associated values.

After NODE_COORD_SECTION, I read node coordinates as (index, x, y) triples and validate numbering from 1 to n+1. Then comes DEMAND_SECTION with (index, demand) pairs where service times default to zero for the depot or use the provided service time data. The distance matrix uses Euclidean rounding if needed. Finally, DEPOT_SECTION confirms the depot is node 1 and ends with EOF. For demands, I parse the node index token but focus on the demand value itself.

Now I'm setting up a tokenizer to split the remaining content by whitespace. For parameter construction, I need to replicate the C++ defaults: durationLimit and vehicleCapacity both default to 1e30, with a check that capacity was actually set. The coordinates are considered provided if demands, x, and y arrays all have the same length. For polar angles, I'm computing `atan2(dy, dx)` scaled by 32768/π and casting to i32 (which truncates toward zero like C++), then applying positive_mod. The depot itself has atan2(0,0) = 0. For vehicle count, if not specified, I default to ceiling(1.3 × totalDemand / capacity) + 3, using an Option type to handle the CLI parameter.

I'm also computing maxDist across all pairs including the depot, and maxDemand across all clients (looping through demands including the depot's demand, which is normally 0). For correlatedVertices, I'm building proximity-sorted neighbors for each client using a BTreeSet to maintain sorted order, keeping the closest nbGranular neighbors. Then I'm applying safeguards: maxDist and maxDemand must be in [0.1, 100000], nbVehicles must be at least ceil(totalDemand/capacity), and penaltyCapacity is clamped to [0.1, 1000] using the formula max(0.1, min(1000, maxDist/maxDemand)).

I'm replicating all the verbose "----- ..." messages and the parameter printing function. For time limit checks, I'm using Instant to measure elapsed seconds as f64. The genetic loop calls managePenalties and printState at iteration 0 and then every nbIterPenaltyManagement and nbIterTraces iterations respectively. The main function mirrors the C++ flow including error handling with "EXCEPTION | ..." messages, and CLI parsing validates that the argument count (including program name) is odd, between 3 and 35, then parses flags like -round via integer conversion.

For the C interface, I'm skipping the FFI layer and instead designing a library API in lib.rs that exposes a `solve` function for idiomatic Rust usage. I'll create the project at `HGS-CVRP/rust/` with package name `hgs_cvrp` and a binary named `hgs`. I need to validate against test instances and create a README documenting the design decisions.

Before running anything, I should check the environment with `uname` and verify cargo is available, since the setup looks like Windows with a shell environment. If the C++ version can be built, I could compare outputs on the same instance with matching seeds to validate correctness — though RNG differences between libstdc++ and MSVC STL might cause divergence on Windows. At minimum, I'll verify solution feasibility and check that costs are close to known benchmarks like the X-n157-k13 optimal of 16876.

Now I'm planning the Rust modules, starting with an RNG implementation that replicates C++ std::minstd_rand and libstdc++'s uniform_int_distribution and shuffle algorithms so the random sequences match the reference implementation.

Defining the RNG structure... Writing the distribution algorithm... I need to verify the early return guard exists and handle the modulo operations carefully with the raw RNG values. For the Rust implementation, I'll expose a `next_u32()` method and replicate the C++ shuffle logic, being mindful that unsigned modulo operations work consistently across platforms. Then I'll organize the modules—keeping `params.rs` for the algorithm parameters to match the naming scheme, alongside `rng.rs` for the random number generator and shuffle, `circle_sector.rs` for the geometry, and `lib.rs` for re-exports and error handling. I'll use a simple `Result<T, String>` type alias for error handling to keep parity with C++'s string exceptions and avoid ceremony. Now I'm defining the Matrix struct with indexing support for ergonomic access to distance values, and I'm sketching out how Params will expose a `dist()` method for retrieving distances.

Defining the core data structures: AlgorithmParameters holds all the algorithm settings as a copyable struct with sensible defaults, Client represents a single customer with coordinates and demand, and Params bundles everything together including the clients vector and the distance matrix that Params owns directly.

For penalties, I'm deciding whether to compute them upfront in Params::new or provide a separate method—since they depend on maxDist and maxDemand which are already stored in Params, I'll add an initial_penalties method that the genetic algorithm can call when it needs them.

Now I'm defining the Individual struct with evaluation metrics like penalized cost, feasibility, and route data, plus methods to create random individuals and evaluate their complete cost. For the file-loading constructor from the C++ version—it's not actually used by the executable, so I'll skip it to keep the scope manageable and note it in the README.

The RNG consumption order matters for sequence parity with the C++ version. When constructing the Genetic struct, the Population gets created with bestSolutionRestart and bestSolutionOverall, each consuming a shuffle from the RNG, followed by offspring—that's three RNG draws before the main population generation even starts. I need to replicate this exact construction order in Genetic::new, and when restart() is called, it creates a new bestSolutionRestart which also consumes an RNG draw. Getting this sequence right ensures the random number stream stays synchronized with the original.

For the Population struct itself, I'm using an arena-based storage with a Vec to hold individuals and their metadata, plus separate tracking for feasible and infeasible solutions. Each individual also maintains a proximity list—distances to other individuals sorted by distance—which I'll insert into deterministically by placing new entries after all existing ones with equal or smaller distance. The add_individual method will update the feasibility lists, allocate an ID in the arena, and compute distances to every other individual in the population.

When adding, I find the insertion position in the subpopulation by scanning backward from the end until I hit someone with lower cost, then insert there. If the subpopulation exceeds the size limit, I trim the worst individuals by biased fitness—being careful that the newly added individual itself might get removed. For best solution tracking, I compare the penalized cost against the restart best; if it's feasible and better, I clone it into both the restart and overall best, recording the timestamp. I'm using clone_from to reuse allocations where possible, though the derived Clone implementation will just do a full clone anyway.

Now I'm implementing removeWorstBiasedFitness, which updates biased fitnesses for the subpopulation, finds the worst individual (skipping the best at index 0), checks if it's a clone by distance, and removes it from the vector along with its proximity entries. I also need to implement updateBiasedFitnesses and getBinaryTournament, which samples two individuals from the population, updates their fitnesses, and returns the one with better biased fitness.

For getBinaryTournament, I need to handle the edge case where the population might be empty — the C++ code would have undefined behavior, but I'll add a debug assertion to ensure at least one individual exists after generatePopulation, since the loop always runs at least once.

Then I'm implementing manage_penalties to replicate the fraction logic and bounds checking, recomputing infeasible costs, and doing a bubble sort by cost with epsilon comparison — I need to match the exact C++ bubble sort structure to preserve the ordering behavior where near-equal values stay in place. Finally, I'm adding get_best_feasible and get_best_found methods.

For the output functions, I'm replicating the print_state formatting with the same column widths and decimal precision, handling getDiversity returning -1.0 for empty populations, and implementing the export functions — exportSearchProgress and exportCVRPLibFormat — where I'll return a Result type and print error messages in main to match the C++ behavior of printing and continuing on file open failures. For cost formatting, I need to be careful about precision: integer-rounded instances print as integers, while floating-point costs preserve their full precision in Rust output.

Now I'm moving to the Split struct, setting it up with flat or nested vectors for the DP matrices, and deciding how to pass penalty values through the propagate and dominates methods — I'll store them as fields in the struct during general_split calls rather than threading them through every method signature.

For the deque implementation, I need to handle the tricky case where back can underflow to -1 in C++ when popping from an empty queue. With Rust's usize, I'll use an exclusive-end approach with front and back_plus1 to avoid underflow issues while keeping the operations straightforward.

Now I'm verifying the queue state across different code paths—checking whether the queue can actually become empty during the for-loop iteration and whether push_back is guaranteed to happen when needed.

For the implementation, I'm planning to create a TrivialDeque that mirrors the C++ behavior. In generalSplit, I need to calculate maxVehicles as the maximum of nbMaxVehicles and the ceiling of total demand divided by capacity. Then I'll loop through clients to populate cliSplit from the chromosome, compute the next duration value, and accumulate sums. In splitSimple, I'll replicate the potential and predecessor matrix logic from row 0, handle the duration-constraint branch and the deque branch separately, check if the final potential exceeds the threshold, and finally clear and refill the chromR rows—iterating from the top down to clear unused vehicle routes and then filling from maxVehicles-1 downward. I need to verify that potential[k][0] is never read for k≥1 since it's left uninitialized, and the duration branch only accesses potential[k][i] starting from i=k. After the propagation loop, I find the minimum cost across all vehicle counts and corresponding route count, then fill the chromosome with the predecessor chain. If no valid solution reaches the final node, I throw an error that bubbles up as a Result. The individual then gets evaluated with penalties included.

Now moving into the LocalSearch struct, which holds the state for local search operations — it tracks the current move context with node and route indices, loads, service times, and penalties for capacity and duration violations.

The `run` method takes a mutable individual and applies local search with the given penalties, but there's a problem: the correlated vertices lists are being shuffled in place during the search, which mutates the supposedly immutable `Params`. I need to extract `correlatedVertices` out of `Params` entirely since it's only used within `LocalSearch` anyway — the `Params` constructor builds it, and only `LocalSearch` consumes it. The cleanest approach is to have `Params::new` compute it but then transfer ownership to `LocalSearch`, either by cloning once (acceptable at ~320KB) or by using `mem::take` to move it out when `Genetic::new` is called, which would require `Params` to be mutable at that point but then reborrow it as immutable afterward.

Actually, I can skip putting it in `Params` altogether — compute the granular neighbors directly in `LocalSearch::new` from a helper function in the params module, keeping the algorithm logic near its documentation while avoiding the mutation problem entirely. The C++ constructor builds it before validation anyway, so there's no observable difference if I move that computation elsewhere.

Now for the main loop: I need to replicate the exact flow with loop IDs and search completion tracking. The tricky part is the RI loop where `nodeU` gets re-read each iteration through its correlated vertices, and after a successful move the route might change — but since I'm tracking by node ID rather than reference, accessing `self.nodes[self.node_u].route` fresh each iteration handles that correctly. When a move succeeds, I'll use labeled `continue` to skip to the next correlated vertex, mirroring the C++ behavior. There's also a special case where if the previous node is the depot, I need to adjust which node I'm examining.

For the empty-route block, I'm setting `node_v` to the depot start node of the smallest empty route, then running moves 1, 2, 3, and 9. The SWAP* block uses route ordering and overlap checks before calling the swap operation, which modifies routes but the outer loop continues with the same route ID and reads fresh state. The `getCheapestInsertSimultRemoval` function returns a cost and optional position tuple instead of writing to a reference.

Now I'm replicating `preprocessInsertions` which updates removal costs and insertion positions for each route pair, iterating through nodes while mutating the data structures via index access rather than pointers. The `insertNode` and `swapNode` operations translate pointer manipulations into index-based surgery, and `updateRouteData` maintains empty route tracking, barycenter calculations, and sector initialization—only touching the sector for non-empty routes since it's only used when checking overlaps. The barycenter uses `atan2` with the average coordinates relative to the depot. Finally, `loadIndividual` and `exportIndividual` replicate the load and export logic with sorting.

For the `run` method signature, I'm passing both `ls_penalties` and `eval_penalties` separately rather than hiding a repair factor, keeping the two penalty sets explicit and documented. The `ThreeBestInsert` structure uses an array of optional locations paired with costs, where `compareAndAdd` shifts entries and overwrites positions as better candidates are found.

In `getCheapestInsertSimultRemoval`, I'm replicating the exact logic: checking the three candidate positions in order, falling back through them if they're adjacent to the target vertex, and finally comparing against the removal cost. There's a subtle quirk where the code sets `found=true` unconditionally after checking `loc[2]`, even if that location is adjacent to the vertex—I need to preserve that behavior for compatibility.

For the move operations, I'm translating each one directly from the C++ formulas, being extremely careful with the cost calculations and the route updates. Move7 is particularly intricate with its in-place reversal logic—I need to replicate the pointer surgery exactly, including the index manipulation in the reversal loop.

For move8 and move9, I'm handling the longer node reassignment sequences and route transfers. The key distinction is that move8 uses a strict `>= 0` pruning check while later using `> -MY_EPSILON`, and move9 doesn't involve reversal—it just swaps edge endpoints while preserving node directions.

I'm also tracking that emptyRoutes is a BTreeSet, so I'll use `.first().copied().unwrap()` to get the first empty route. And I need to be careful distinguishing between pointer comparisons (node identity) and index comparisons (client indices), since nodeY could be a depot with index 0 while nodeU never is.

Looking at the condition checks across all moves: move1 compares client indices, move2/3/5/6 use node identity, move4 compares both client indices and checks adjacency via cour values, move7 checks node identity for the next pointer, and the RI loop checks if a node's predecessor is the depot. The swapStar function compares route cour values.

Now I'm setting up the Genetic struct with its fields including params, split, local_search, population, offspring, rng, and penalty configuration.

Defining the struct methods... Setting up main.rs to parse command-line arguments, read the CVRPLIB instance file, initialize the genetic algorithm solver, and handle output—printing results or exceptions with exit code 0 to match C++ behavior. For the instance file structure, I'm defining a struct to hold coordinates, distance matrix, service times, demands, and constraints, then implementing a reader that parses the first three lines separately and tokenizes the remaining data by whitespace. Writing format helper... I need to verify the minstd_rand implementation with a unit test checking that the 10000th value matches the standard (399268537 for seed 1), and I'll add basic tests for CircleSector operations and positive_mod. For the `nb_moves` variable, I'm debating between i32 (matching C++ exactly) and i64 (safer for very long runs), but since both implementations have the same overflow risk at 2^31 moves, I'll stick with i32 to maintain exact parity with the C++ version. Defining CircleSector struct... The C++ code assigns the loop variable to a member that persists after swapStar returns, but since nothing depends on that state and the RI loop reassigns before use, I can stick with local variables. For Population::new, I'm taking params and an RNG reference to construct the initial best solution and feasibility lists via random shuffles. getBestFound returns an Option wrapping a reference to the best Individual if its cost is below the penalty threshold. addIndividual should only return true when the solution is both feasible and better than the restart best. For managePenalties, I'm implementing a bubble sort over the infeasible population IDs by comparing their penalized costs, being careful with the loop bounds to avoid underflow when the population is empty.

Now I'm handling the print formatting — converting C++ printf specifiers to Rust format strings, where %6d becomes {:6} and %.2f becomes {:.2}. For the final algorithm completion message, I need to match C++ cout's default float formatting, which is roughly equivalent to printf's %g with 6 significant figures, so I'm writing a small helper function that formats floats accordingly.

Writing format helper... Testing edge cases with the format_g function — checking how values like 0.2, 0.85, 1.2, and larger numbers like 16876 and 104678.18 format correctly, verifying they match C++'s %.6g behavior by calculating the right number of decimal places based on the exponent and stripping trailing zeros appropriately.

Now I'm thinking about the random number generation interface — I need to expose a `uniform_below` function that generates values in [0, n-1] for the various places where the C++ code uses uniform_int_distribution, like for selecting clients in crossover or participants in tournament selection. I'll define an internal `uniform_inclusive` helper that handles the [0, b] range, then build the public API around that.

For the shuffle implementation, I'm replicating the libstdc++ algorithm exactly to maintain RNG sequence parity — even degenerate cases like a population of size 1 still consume random numbers from the engine, which matters for reproducibility. The shuffle logic uses a specific formula to map uniform random values into valid swap positions.

Implementing shuffle with the correct algorithm... Checking the boundary conditions for the two-uniform-ints optimization — when swap_range reaches urange, the product swap_range*(swap_range+1)-1 stays within URNG_RANGE, so the downscaling path is safe even at the limits. For the actual slice sizes we're dealing with (up to 1001), this is trivial. Now I need to implement the uniform draw itself by extracting the raw engine output, subtracting the minimum, and scaling down by the appropriate factor, then handle the tournament selection using a uniform distribution over the total weight.

For crossover, I'm reusing the same distribution to draw start and end points, redrawing if they're equal. The genetic algorithm's random decisions like the 50/50 coin flip use direct modulo operations on the engine output, and the granular shuffle decision checks if a modulo result equals zero. I need to be careful about the division-by-zero case when nbGranular is zero, though that's undefined in C++ too. Finally, I'm matching the exact RNG call sequence from the C++ version — the Genetic constructor initializes the split and local search objects without drawing, then the population constructor triggers shuffles during best solution initialization and offspring generation, followed by the main loop where each individual gets shuffled and potentially has granular decisions made.

For memory and performance, the distance matrix is flattened for better cache locality, and all scratch buffers are preallocated like in C++. The ranking vector in updateBiasedFitnesses and the frequency vector in crossover are allocated per call, matching C++'s pattern. Individual construction also allocates, which happens once per population member during generation. For the result type, I'll use a simple type alias with String as the error type to match the C++ string-based error handling.

Now I'm setting up the error handling flow: Split::general_split and Genetic::run both return Result types to propagate errors from population generation and crossover operations. Params::new and instance reading also return Results with validation checks. The main function wraps everything in a run() call that catches errors and prints them with an "EXCEPTION" prefix, always exiting with code 0 for parity. For command-line parsing, I'm mirroring the C++ structure with a CommandLine struct holding algorithm parameters, vehicle count, file paths, and flags. The argument count check mirrors C++'s validation, and I'm using Rust's parse() instead of atoi to properly handle invalid inputs rather than silently defaulting to zero.

For the Params struct, I'm accepting an optional vehicle count (None triggers default computation) and handling the swap-star coordinate logic with a boolean flag that gates coordinate setup and verbose output. The genetic algorithm's time limit check uses exact float comparison against 0.0, and elapsed time is computed from Duration in seconds. For output formatting, I'm using a format_g function to match the C++ precision (6 significant figures) for both elapsed time and cost values in the progress CSV, which logs instance name, seed, cost, and time per line.

On naming, I'm keeping the move1 through move9 identifiers with documentation comments for cross-referencing with the paper and original C++ code, even though Rust convention would prefer more descriptive names. Now I'm setting up the LocalSearch run loop, which initializes penalties, loads the individual, shuffles node and route orderings, and conditionally shuffles correlated vertices based on granularity. The main search loop then iterates through moves, starting with relocation operations on each client node.

For the correlated vertices access, I need to be careful with borrowing — I'm capturing the count before the loop and accessing by index to avoid holding a borrow while calling move functions. Since client node IDs directly correspond to their route indices, I can simplify to `self.correlated_vertices[node_u]` with a clarifying comment. Then I'm setting up the move evaluation by storing the current node pair and their routes, checking modification timestamps to decide whether to attempt the move.

For the empty routes block, the C++ `continue` statements jump back to the outer position loop, skipping any remaining attempts in that block. In Rust, I need to use a labeled break or restructure to achieve the same effect — when a move succeeds, I want to skip to the next iteration of the outer loop rather than trying additional moves.

Now I'm looking at the SWAP* block, which iterates through vehicle pairs and checks conditions like whether both routes have customers and whether the route indices satisfy the ordering constraint.  The key insight is that `route_u` gets set before the inner loop, and even though `swap_star()` may modify routes, the route ID itself stays consistent. I also need to make sure `route_u` and `route_v` are set before calling `swap_star()` since that method depends on them. The `loop_id` counter increments at the end of the main loop and stays local since it's only referenced within the `run()` method body. Now I'm defining the `set_local_variables_route_u` helper function, which extracts the route index from a node and caches related indices like the next node and previous node positions.

Continuing with move translations... Writing helper methods... Building the linked list for each route by connecting clients in order, then updating route metadata and resetting cached values for optimization checks. After that, I'm exporting the individual by sorting routes by their polar angle barycenter.

Continuing with the swap_star function... Finishing the relocate-V case and then checking if the move improves the solution — if the cost is positive, we reject it and return false, otherwise we insert both nodes at their best positions, update the route data, and mark the search as incomplete. I'm also noticing a detail in the C++ version where the relocate-only cases use the straight best insertion position without the simultaneous-removal adjustment, which differs from the swap case where both positions are computed together.

Now I'm implementing the cheapest_insert_simult_removal function, which returns both the cost and the position where a node should be inserted. The function checks the precomputed best insertion locations in order, skipping any that would place the node adjacent to the node being removed, and falls back to the previous node in the route if needed.

Defining the insertion logic... Initializing the three-best-insert structure for route r2 and client u, starting with the depot insertion cost, then iterating through all nodes in r2 to find the three best positions where u could be inserted, comparing each candidate position's cost and updating the structure accordingly. Translating the move8 pointer operations to Rust... Finishing the conditional branch and verifying the logic against the C++ implementation — checking that node_x retains its original value even as the cursor moves, confirming that the cost validation happens before any structural changes, and noting that move8 has no redundant adjacency checks since the routes are guaranteed to be different. Now I need to replicate move9, which reads the depot predecessor before relinking and then updates routes on the tail nodes before reconnecting. Handling the tail swaps in both branches — when routeU's tail is empty versus when it has clients. The key insight is that Y becomes part of routeU and X becomes part of routeV, with the depot pointers getting rewired accordingly. In the empty case, Y is actually routeV's end depot, so the linkage needs special handling to avoid breaking the route structure. The empty route case works through the same code path, which is elegant — the depot node Y doesn't trigger reassignment, and the tail clients get properly rerouted. The depot final nodes stay bound to their original routes as intended. I need to preserve the exact order of operations, particularly reading `depotUpred` before any relinking happens.

For move7, the intra-route checks use node positions that are lazily updated after each move, so they're fresh. The position comparison happens before the cost calculation, then the adjacency check comes after — I need to keep that order for correctness.

Now looking at the Population struct, which holds the search state with items, free slots, feasibility tracking across load and duration constraints, search progress metrics, and best solutions tracked both per restart and overall.

Implementing the biased fitness update requires careful borrow handling — I'm collecting ranking data from one subpopulation list while needing to write back to the items field, so I'll capture the IDs first to avoid conflicting borrows. Finishing the biased fitness formula — when the population size exceeds the elite threshold, I blend the fitness rank with a diversity component scaled by the proportion of non-elite individuals. For the `remove_worst_biased_fitness` function, I'm updating biased fitnesses first, then finding and removing the worst individual by a specific criterion: clones (those with near-zero closest-neighbor distance) are prioritized for removal, and among non-clones or among clones, I pick the one with the lowest biased fitness. I'm deciding to use `panic!` for the invariant violation case where we'd eliminate the last individual, matching the C++ behavior of exiting on an exception, rather than propagating a `Result` through the call chain.

Now I'm handling the cleanup after removal: deleting the individual from the subpopulation vector and removing all proximity entries pointing to that individual from the remaining members' lists, then marking its slot as free. The tricky part is the `average_broken_pairs_distance_closest` function — when proximity is empty (single-member subpop), dividing zero by zero gives NaN, but that's okay because the NaN gets discarded before sorting when the subpop size check catches it, and for larger subpops every member has at least one proximity neighbor. In Rust, I'll return `f64::NAN` explicitly when maxSize is zero.

For the binary tournament selection, I'm picking two random positions from the combined feasible and infeasible lists, mapping them to actual individual IDs, updating biased fitnesses for both, then returning whichever has the lower fitness value. The random draws match the C++ behavior of calling the engine twice.

Still need to replicate the penalty management logic and the print function.

For the restart flow, I'm splitting the work so that `reset_for_restart` handles printing the reset message, clearing the population, and generating a new random restart-best individual, then calling `generate_population` afterward — all gated on the verbose flag.

I've got accessor methods for `best_feasible`, `best_infeasible`, and `best_found`, where the first two are internal helpers used only in printing and their Option-ness drives the "NO-FEASIBLE" output.

Now I'm sketching out the main `run` loop...

Implementing the core GA loop... Continuing the crossover function, I'm handling the borrow checker carefully by copying the route count from the parent before the split call, which lets me release the parent borrow and then mutate the offspring. Then I'm moving into the population generation function, which builds the initial population by creating random individuals up to a target size, applying the split operation to each one, and respecting the time limit if one is set.

For individuals that aren't feasible after the initial repair, I'm running local search again with amplified penalties to push them toward feasibility, then adding them to the population if they succeed. I need to be careful about the penalty values — the split operation uses the standard penalties, but local search can use different penalty multipliers depending on whether we're in the repair phase or final evaluation. The evaluate_complete_cost function will need to take both the parameters and the penalties as arguments so it can compute the full cost correctly.

Now I'm iterating through each route in the chromosome, calculating the distance by summing edges from the depot through all customers and back, tracking the load and service time accumulation, and maintaining predecessor/successor pointers for the route structure. Once I've processed all routes, I'm computing the total penalized cost by adding the distance to the weighted excess violations, then checking feasibility against an epsilon threshold.

I need to define EPSILON as a constant matching the C++ value of 0.00001, and I've spotted a precision issue with PI — the C++ code uses 3.14159265359 while Rust's standard PI is more precise at 3.141592653589793, which could cause slight differences in polar angle calculations. I'll use the C++ constant for parity. The polar angle computation in params is the only place PI is used, so that's the only place I need to apply this constant.

Now I'm working on the population management, starting with the add_individual method that tracks feasibility.

Handling the borrow checker for proximity calculations... I need to inline the cost calculation directly instead of calling a method, since `self.cost_of()` would create a shared borrow that conflicts with the mutable borrow on `subpop`. After inserting the new individual, I'm checking if the subpopulation exceeds the size limit and removing the worst individuals if needed, being careful to drop the borrow before calling removal methods. Then I'm tracking the best solution found, updating both the restart-best and overall-best if this individual improves them.

For allocation, I'm reusing free slots when available, otherwise appending to the items vector. When removing the worst individual, I need to clean up its proximity references from all remaining members in that subpopulation—iterating through indices and reading each member's ID fresh each time to avoid borrow conflicts while mutating the items array. There's an edge case where a newly added individual could be immediately removed by survivor selection, which is fine.

Now I'm setting up the Split struct with a 2D distance matrix initialized to a large value, and implementing the general_split function that preprocesses the chromosome into client split data—computing cumulative demands, service times, and distances. The logic checks if a simple split succeeds; if not, it falls back to a more complex split algorithm, then evaluates the complete cost.

For the dynamic programming phase, I'm implementing the propagate function that computes potential values using the distance matrix and penalties, along with dominates checks to maintain a deque of candidate predecessors. The split_simple branch initializes a deque and iterates through clients, computing potentials from the front of the queue while pruning dominated states from the back before adding new candidates.

I'm considering whether to preallocate the deque as a field in Split rather than allocating fresh each call—this would save allocations without changing behavior. The tricky part is managing borrows: the deque field needs &mut self for operations like pop_back, but propagate and dominates_right take &self. Since I'm only copying values from the deque (front, back return usize copies) and making sequential method calls without holding references across them, the borrow checker should be satisfied.

Now I'm assembling the Genetic struct with its constructor, initializing the RNG, Split, LocalSearch, Population, and a random offspring individual, then storing the initial penalties from the parameters.

Setting up main.rs with command-line parsing, instance loading, and the solver workflow — after running the genetic algorithm, if a best solution exists, I export it in CVRPLIB format and write the search progress to a CSV file. The borrowing works out since best_found() returns a shared reference to an Individual from the population, and the export methods take shared references.

Now I'm working through Params::new to match the C++ initialization order exactly — computing the number of clients, checking if coordinates are provided, building client data with polar angles when needed, determining fleet size (either from user input or calculated with a formula), computing maximum distances, building the correlated distance matrix, running safeguards with specific error messages, and handling penalty initialization. The key detail is that penalties should be stored as initial values in Params so they can be accessed later.

For the file reading, I'm implementing the parsing logic to skip the first three lines and then tokenize the remaining content by whitespace, mirroring how the C++ code reads the instance file. Parsing the match statement for header tokens like DIMENSION, CAPACITY, and DISTANCE to extract vehicle and problem parameters, then validating that node numbering is sequential and checking for required sections like DEMAND_SECTION before computing the distance matrix using Euclidean distances.

Now handling the DEPOT_SECTION validation — expecting depot index 1 followed by EOF, with proper error messages if the format is wrong. I'm defining a lightweight Matrix struct that stores distances in a flat vector with inline accessors for getting and setting values, and considering whether to add an Index trait implementation or a custom constructor for asymmetric matrices. For the library structure, I'm organizing modules for algorithm parameters, genetic operations, and instance parsing.

Setting up the package with a library crate `hgs_cvrp` and a binary `hgs` that uses it, keeping the edition at 2021 with no external dependencies. The local_search module will be split into submodules for different components like Node, Route, and ThreeBestIn structures.

For privacy within the module tree, fields in local_search/mod.rs are accessible to child modules like moves.rs and swap_star.rs since they're all part of the same module subtree. I'll add clippy-style quality checks to avoid warnings, include doc comments for key functions, and write a README documenting the design decisions around RNG parity with libstdc++ and the index-based list approach.

The main deviations from the C++ version are the RNG implementation matching GCC's minstd_rand, fixing the tournament selection evaluation order, using wall-clock time instead of CPU clock, emulating cout's double formatting, and handling errors as Results rather than exceptions. I'm also making LocalSearch own the correlated vertices since it's the only consumer. For validation, I'll run cargo build and test with known-value checks for the RNG and format helpers, then test against an actual instance to verify the solution cost and validity, potentially comparing traces with the C++ version if both can be compiled.

I'm checking edge cases like uniform_below with population size 1, the loop condition evaluation order for time limits, and handling empty feasibility lists—particularly the division by zero case where Rust's f64 division naturally produces NaN instead of panicking. I'm also ensuring VecDeque works correctly for the feasibility lists and that the search progress file is created empty as expected.

Defining the random individual constructor... Checking the correlated shuffle logic where the RNG is drawn for every client and the shuffle happens conditionally. In the RI loop, moves are applied and the loop continues with fresh local variables for the next position. The depot-try block reassigns nodeV to the previous node, then continues with specific moves while skipping others. After a successful move in the first batch, we skip ahead, so nodeV remains unchanged when reaching the depot block. When the depot block executes, it resets nodeV to the depot start of the same route, and set_local_variables recomputes intra-route values (which stay the same since we're in the same route). For the empty-route block, both route variables get fresh values and intra becomes false since the routes differ, so move9 runs unconditionally as guaranteed inter-route.

Now I'm setting up the node ID layout where clients occupy indices 0 through n, depot starts at n+1 through n+V, and depot ends at n+1+V through n+2V. I'm initializing the nodes array with proper depot markers and route assignments, then creating helper functions to compute depot indices from route numbers. I also need to add unit tests for the RNG to verify the default seed produces expected values and smoke tests for shuffle determinism.

For integration testing, I'm planning a minimal end-to-end scenario with a few clients on a line where the optimal solution is known—something like depot at origin with clients at fixed distances, then running the solver with a small iteration budget and asserting the final cost matches the manually computed optimum. I'll keep it simple: maybe 2 clients with capacity constraints that fit in one route, so the expected cost is straightforward to calculate.

I need to be careful about the parameter bounds: maxDist must be ≥0.1, maxDemand in [0.1, 1e5], and with total demand 2 and capacity 10, the lower bound on vehicles is 1, which is valid. This setup exercises the full pipeline—split, local search, population management, genetic operations—as a solid smoke test.

One critical detail: I need to replicate the dominance comparison formulas exactly as they appear in the C++ code, paying close attention to index arithmetic. The dominates and dominatesRight checks involve array accesses like cliSplit[j+1] and sumDistance[j+1], so I need to verify that j+1 stays within bounds when these functions are called.

For the propagate function, the third parameter represents the source row in the potential table, and the result gets written to the next row. In splitSimple, both source and destination are row 0, which is fine since we're applying Bellman-style updates in topological order. The pred array tracks predecessors for path reconstruction, and I need to make sure pred[k+1] is set correctly in both the deque and duration branches.

If splitSimple fails to reach node 0, chromR gets left partially uninitialized, but that's okay because splitLF will overwrite it. However, if splitLF also fails to reach 0—

which could happen if maxVehicles is too small—the function returns 0 without anyone checking it, leaving the individual potentially invalid. The C++ code has the same behavior, so I'll replicate that exactly. The pred chain construction should guarantee we reach 0 by design, though there's a subtle risk of revisiting nodes; I'll trust the C++ approach and not add panic checks.

For the reconstruction loops, I need to avoid usize underflow by iterating k in reverse from nbRoutes-1 down to 0, and in splitSimple the loop goes from maxVehicles-1 down to 0. Since maxVehicles is at least 1 (guaranteed by the demand safeguard), this is safe. The potential array initialization sets row 0 to [0, 1e30, 1e30, ...] in both split functions, and splitLF continues initializing the remaining rows. I'm verifying that potential[k][k] gets written unconditionally in round k-1 when the deque branch executes, so even if it remains 1e30, the loop condition in the duration branch catches it and skips propagation. Now I'm double-checking the move operations—move5 and move6 have the correct swap and insert order—and confirming that exclusion conditions are evaluated after cost checks for proper floating-point guard semantics.

For move1, the structural check comparing nodeU and nodeY is equivalent to checking if they're the same client since courses are unique per client. Move4's check against V's predecessor works because the predecessor could be the depot. I also need to handle the LocalSearch struct borrowing self.params while mutating self fields, which is fine since params is a shared reference. The correlated_vertices field needs to be owned and built during LocalSearch initialization—I'll create a helper function in the local_search module to compute this from the params.

Building the correlated vertices by sorting each client's distances to others and keeping the nearest neighbors...

Checking edge cases: n=1 gives n-1=0 so take(0) safely produces an empty set, no underflow issues. Now I need to set up the environment and write the files. I should also document the zero-cost abstraction approach in the README—using index arenas and Copy types without unnecessary Rc/RefCell allocations except where semantically required for population copies.

Let me verify the RNG consumption order matches the C++ implementation: Population::new initializes bestSolutionRestart before bestSolutionOverall following declaration order, and the Genetic struct members are ordered correctly with params, split, localSearch, population, and offspring. The only RNG seed call happens in Params::new, which aligns with the C++ constructor. Everything checks out, so I'm ready to start writing the code.

For the export functions, I'm handling the CVRP format output by iterating through routes and skipping empty ones—since they're sorted by polar angle and contiguous at the front, the numbering naturally becomes 1..nbRoutes. For file I/O errors, I'll collect the output into a String and use fs::write, printing the "impossible to open" message on failure. For search progress export, I'll silently ignore write errors to match C++ behavior where the ofstream would just fail silently.

Before writing code, I need to confirm the Population struct carries a reference to Params for accessing nbClients, algorithm parameters, start_time, verbose, and penalty bounds. The print_state function takes iteration counts and penalties, and I'm replicating the exact C++ format string spacing with `{:6}` for usize values. For the local search RI loop, the timing is critical—nodeU's whenLastTestedRI gets set before the inner loop, and after successful moves, route modification times update via update_route_data with the move counter incremented beforehand to match C++'s ordering.

Now I'm implementing the swap_star relocate logic, carefully replicating the C++ structure where I iterate through nodes in routeU, look up precomputed best insertion positions in routeV, calculate the distance deltas for both routes, and then compute the total move cost including all penalty contributions before checking if it improves the best move found so far.

I'm handling the borrow checker by extracting the cost and location values early to drop the shared borrow before updating the best move. I also need to verify that swap_star is only called with non-empty routes and that SwapStarMove defaults to a high cost with None values for positions.

For the final file structure, I'm organizing everything across lib.rs, utility modules like rng and matrix, core data structures like instance and individual, the local search components split into separate files, and the genetic algorithm orchestration, plus formatting utilities.

I'll name the formatting module `fmt.rs` since it's a contained module that doesn't conflict with the standard library's fmt module when accessed via the full path. The lib.rs will re-export the main public types and define a Result type alias and EPSILON constant. I'm setting the rust-version to 1.70 to ensure compatibility with features like BTreeSet::first, and I need to make sure the LocalSearch loop properly updates the whenLastTestedSWAPStar timestamp after checking the last test value.

Now I'm verifying the geometric calculations: CircleSector::overlap is static, polar angles are i32, and the positive_mod function handles wrapping correctly. For update_route_data, the barycenter calculation uses atan2 with the y-component first, and when coordinates aren't available or swap-star isn't enabled, the polar angles default to 0 which makes sectors degenerate but still allows sorting by radius. I'm also confirming that Split stores both capacity and duration penalties since different branches use different penalty types, and the initialization creates the right-sized vectors with proper defaults.

Now checking the command-line argument parsing: argc semantics match between C++ and Rust (both include the program name), and for numeric arguments I'm parsing directly as f64 or i32 with error handling for invalid values. The seed argument parses as u32, and while C++ atoi silently converts negatives to large values, I'll just reject them with a clear error message since that path shouldn't occur in normal usage. I'm also replicating the help text display exactly as it appears in the C++ version.

For the genetic algorithm's timing check, I'm using an inline closure to evaluate whether the elapsed time has exceeded the limit. The local search moves need careful implementation—moves 4 through 9 each have specific cost formulas involving distance lookups and cumulative reversal penalties that I'm replicating verbatim from the source. The structural guards for move2 and move3 check for depot nodes and conflicting positions. Finally, the Genetic struct exposes its population publicly so main can access the best solution found, while other fields remain private, and I'm adding a method to Population that returns the best individual if its cost is below the feasibility threshold.

Now I'm ready to write the code, then build and test it against the C++ reference implementation.我已经读完了 C++ 源码，现在先检查环境是否就绪（cargo 可用性、实例文件），然后确认测试基准。

</details>

<details>
<summary>Tool Call: uname -a; which cargo rustc g++ cmake; cargo --version 2>/dev/null; rustc --version 2>/dev/null</summary>

*Status: Completed*

Terminal:
```
Linux LAPTOP-3HQL9PF9 6.18.33.1-microsoft-standard-WSL2 #1 SMP PREEMPT_DYNAMIC Fri Jun  5 01:12:21 UTC 2026 x86_64 x86_64 x86_64 GNU/Linux
/home/username/.cargo/bin/cargo
/home/username/.cargo/bin/rustc
/usr/bin/g++
/usr/bin/cmake
cargo 1.96.0-nightly (888f67534 2026-03-30)
rustc 1.96.0-nightly (55e86c996 2026-04-02)

```

</details>
<details>
<summary>Tool Call: ls Instances/CVRP | head -20; cat Test/TestExecutable.cmake</summary>

*Status: Completed*

Terminal:
```
CMT1.vrp
CMT10.vrp
CMT11.vrp
CMT12.vrp
CMT13.vrp
CMT14.vrp
CMT2.vrp
CMT3.vrp
CMT4.vrp
CMT5.vrp
CMT6.vrp
CMT7.vrp
CMT8.vrp
CMT9.vrp
Golden_1.vrp
Golden_10.vrp
Golden_11.vrp
Golden_12.vrp
Golden_13.vrp
Golden_14.vrp

# delete artifacts from previous runs, if any
file(REMOVE mySolution.sol)
file(REMOVE mySolution.sol.PG.csv)

# solve the given instance
execute_process(
        COMMAND ./hgs ../Instances/CVRP/${INSTANCE}.vrp mySolution.sol -seed 1 -round ${ROUND}
        RESULTS_VARIABLE result
)
message(${result})

# read the result and compare with the given cost value
file(STRINGS mySolution.sol solution)
string(FIND "${solution}" "${COST}" cost_position)

# if not match, throw an error
if(${cost_position} EQUAL -1)
    message(SEND_ERROR "Test error for ${INSTANCE}. Cost != ${COST}")
endif()

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

I've got WSL2 Linux set up with cargo nightly available, so now I'm checking the CMakeLists.txt file to understand what test cases are defined and what the expected costs should be.

</details>

<details>
<summary>Tool Call: Read file `CMakeLists.txt`</summary>

*Status: Completed*

```HGS-CVRP/CMakeLists.txt
     1	cmake_minimum_required(VERSION 3.15)
     2	project(HGS_CVRP)
     3	set(CMAKE_CXX_STANDARD 17)
     4	
     5	set(
     6	        src_files
     7	        Program/Genetic.cpp
     8	        Program/Individual.cpp
     9	        Program/LocalSearch.cpp
    10	        Program/Params.cpp
    11	        Program/Population.cpp
    12	        Program/Split.cpp
    13	        Program/InstanceCVRPLIB.cpp
    14	        Program/AlgorithmParameters.cpp
    15	        Program/C_Interface.cpp)
    16	
    17	if (MSVC)
    18		set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)
    19	endif (MSVC)
    20	
    21	include_directories(Program)
    22	
    23	# object to be used by both static and shared (compile only once)
    24	add_library(objlib OBJECT ${src_files})
    25	set_property(TARGET objlib PROPERTY POSITION_INDEPENDENT_CODE 1)
    26	
    27	# runtime library
    28	add_library(lib SHARED $<TARGET_OBJECTS:objlib>)
    29	set_target_properties(lib PROPERTIES OUTPUT_NAME hgscvrp)
    30	
    31	if(CMAKE_CURRENT_SOURCE_DIR STREQUAL CMAKE_SOURCE_DIR)
    32	    # We're in the root, build everything
    33	    # static library
    34	    add_library(lib_static STATIC $<TARGET_OBJECTS:objlib>)
    35	    # if static and runtime libraries use name "hgscvrp", MSVC will overwrite one
    36	    # of them, because both STATIC and SHARED builds create "hgscvrp.lib"
    37	    set_target_properties(lib_static PROPERTIES OUTPUT_NAME hgscvrp_static)
    38	
    39	    # Build Executable
    40	
    41	    add_executable(bin Program/main.cpp)
    42	    target_link_libraries(bin PRIVATE lib_static)
    43	    set_target_properties(bin PROPERTIES OUTPUT_NAME hgs)
    44	
    45	
    46	    # Test Executable
    47	
    48	    include(CTest)
    49	    add_test(NAME    bin_test_X-n101-k25
    50	             COMMAND ${CMAKE_COMMAND}   -DINSTANCE=X-n101-k25
    51	                                        -DCOST=27591
    52	                                        -DROUND=1
    53	                                        -P ${PROJECT_SOURCE_DIR}/Test/TestExecutable.cmake)
    54	    add_test(NAME    bin_test_X-n106-k14
    55	             COMMAND ${CMAKE_COMMAND}   -DINSTANCE=X-n110-k13
    56	                                        -DCOST=14971
    57	                                        -DROUND=1
    58	                                        -P ${PROJECT_SOURCE_DIR}/Test/TestExecutable.cmake)
    59	
    60	    # Test Executable: Instances with Duration, without Rounding
    61	    add_test(NAME    bin_test_CMT6
    62	            COMMAND ${CMAKE_COMMAND}    -DINSTANCE=CMT6
    63	                                        -DCOST=555.43
    64	                                        -DROUND=0
    65	                                        -P ${PROJECT_SOURCE_DIR}/Test/TestExecutable.cmake)
    66	    add_test(NAME    bin_test_CMT7
    67	            COMMAND ${CMAKE_COMMAND}    -DINSTANCE=CMT7
    68	                                        -DCOST=909.675
    69	                                        -DROUND=0
    70	                                        -P ${PROJECT_SOURCE_DIR}/Test/TestExecutable.cmake)
    71	
    72	    # Test Library
    73	    add_subdirectory(Test/Test-c/)
    74	    add_test(NAME       lib_test_c
    75	             COMMAND    lib_test_c)
    76	
    77	    # Install
    78	    install(TARGETS lib
    79	            DESTINATION lib)
    80	    install(TARGETS bin
    81	            DESTINATION bin)
    82	    install(FILES Program/AlgorithmParameters.h Program/C_Interface.h
    83	            DESTINATION include)
    84	endif()
    85	

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

I'm mapping out the Rust implementation structure, planning to create a workspace in a subdirectory alongside the existing C++ code so both implementations can coexist in the project.

For the RNG strategy, I need to replicate the C++ behavior as closely as possible—implementing the same linear congruential generator and shuffle algorithm to ensure deterministic, consistent results across runs, even though some variance from the original is acceptable.

For the distance matrix, I'll use a flat vector with index helpers rather than nested vectors for better cache locality and Rust idioms. And for the local search linked list, I'll replace C++'s raw pointers with an index-based approach using a vector to store nodes, which gives the same performance without unsafe code.

I'm planning the node arena layout: clients occupy indices 0 through nbClients (with 0 as a sentinel), then depot starts, then depot ends. Each node tracks its client index, position, timing info, links to neighbors, route assignment, cumulative load and time, and removal cost. For the population structure, instead of C++'s multiset of pointers, I'll maintain a sorted vector of (distance, individual-id) pairs for each individual's proximity list, using binary search for insertions.

Tie-breaking by ID is actually more deterministic than C++'s pointer-address approach, and it doesn't affect the algorithm's behavior since equal distances produce the same sums and comparisons regardless of order. I'm designing individuals as arena-allocated items with stable IDs, storing each as a struct containing the individual data, biased fitness, and its sorted proximity list, avoiding Rc/RefCell overhead entirely.

Since proximity references only exist within the same subpopulation, I'm considering whether to use a slab with free lists for stable indexing or just keep a simple Vec with sequence numbers to handle removals. The simplest approach might be boxing individuals and storing them in a Vec, using sequence numbers in proximity lists to remain stable when elements are removed from the middle. Params holds both immutable config and mutable state like penalties and the RNG, so the cleanest approach is to pass `&mut Params` through the call chain rather than storing references—components own their scratch data and take Params as arguments. The Genetic struct wraps Params, Split, LocalSearch, Population, and offspring, with methods threading mutable Params where needed (Split uses immutable borrows since it doesn't touch the RNG, while LocalSearch and Population need mutable access for RNG and penalty updates). This avoids borrow checker conflicts and keeps the design zero-cost.

For the remaining components: Split uses flat scratch matrices sized by vehicle and client counts, Trivial_Deque mirrors the C++ version, and CircleSector uses i32 arithmetic with modulo wrapping. Instance parsing reads tokens from the file—skipping the first three header lines entirely, then looping through tokens like "DIMENSION" and "EDGE_WEIGHT_TYPE" to extract the corresponding values.

After NODE_COORD_SECTION, I parse node coordinates, then DEMAND_SECTION for demands, and finally DEPOT_SECTION which should contain "1" and "EOF". The output format prints routes and costs using default floating-point precision, matching C++'s behavior where 27591 displays as an integer and decimal values like 555.43 print with their natural precision.

For CMT instances with round=0, costs can have floating-point artifacts like 1077.5499999999999, which C++ would format as 1077.55 using %g with 6 significant digits. I need to implement a formatter that mimics this behavior exactly, so I'm writing a function that computes the exponent and formats accordingly. Finishing the format function...

For the CSV export and verbose logging, I need to match C++ printf formats like "%.2f" for two decimal places, which translates directly to Rust's `{:.2}` formatting. The printState function uses formats like "%6d" for spacing, which I can replicate with `{:6}`.

On timing, C++ uses `clock()` for CPU time while Rust's `Instant` gives wall time. They're close enough for behavioral parity, though ideally I'd match the CPU time measurement more precisely.

For error handling, I'll use Rust's Result type with String errors, mirroring how C++ throws std::string exceptions that main catches and prints as "EXCEPTION | ...".

On the C interface layer, the original project provides FFI bindings for other languages. Rather than replicating that directly, I'll structure the Rust code as a library crate with a clean Rust API, which is the idiomatic equivalent. The C FFI could be added later as an optional feature if needed, but for now I'll focus on the core library and binary to keep scope manageable.

For the route ordering in exportIndividual, I need to sort by polar angle barycenter. Since the angles are computed via atan2 and won't produce NaN, I can safely sort tuples of (angle, index) using total_cmp for the float comparison and then comparing indices as a tiebreaker.

The managePenalties function uses a bubble sort with epsilon comparison, so I'll replicate that exact algorithm rather than using a standard sort—the epsilon threshold behavior is important for matching the original ordering.

For addIndividual, I'm replicating the insertion logic that walks backward while cost exceeds the new cost minus epsilon. And for getBinaryTournament, I need to generate two uniform random integers; I'll implement a clean rejection-sampling approach for the uniform distribution rather than relying on modulo bias.

For the minstd_rand generator itself, I'm implementing the linear congruential formula with proper seed handling—if the seed is zero or a multiple of the modulus, it maps to 1, matching libstdc++'s behavior. Implementing the RNG struct...

For the rest of the algorithm, I'm using direct modulo operations on the RNG output for things like checking granular search conditions, and I'm handling numeric types by using usize for indices and f64 for demand/duration values. For the correlated vertices structure, I'll build it from sets by collecting into a vector, sorting, and deduplicating to match the C++ behavior.

The local search is the complex part — I'm setting up an index-based arena system where nodes are stored in a flat array with depot sentinels at fixed positions, and I'm replicating the C++ pointer-heavy logic using indices and helper functions instead. Each node tracks its position in the route, load, time, and other state needed for the moves, and I'm storing the move context as fields on the local search struct to mirror how C++ handles it.

For the implementation details, I need to handle index dereferencing carefully — things like checking if a node's predecessor is a depot become array lookups instead of pointer checks. I'm passing the distance matrix and penalty parameters as method arguments since storing references on the struct would create lifetime complications with the genetic algorithm's ownership model. The `bestInsertClient` structure is a 2D vector of insertion candidates, where each entry tracks the best positions to insert a client into a route, and I need to handle the case where no valid insertion exists. be set while bestPositionU is null? In the relocate-V-to-routeU block, U stays null but bestPositionV gets set, whereas the first block sets both. Since swapStar only runs between non-empty routes and getCheapestInsert always finds at least one valid position, both positions should be non-null in the first block. In the relocate blocks, bestPosition comes from bestLocation[0] which is guaranteed non-null after preprocessing. The key thing is that when U and V are in different routes, there's no self-insertion issue, and the move cost uses bestCost[0] correctly. In the C++ implementation, if the best move is a relocate-V-to-U, bestPositionU stays null so only V gets inserted.

Now I need to replicate the preprocessInsertions side effect that computes deltaRemoval for all nodes in the first route, and handle the getCheapestInsertSimultRemoval logic with its bestLocation[1] checks carefully. For the swapStar iteration, I'm using local variables for u and v instead of member fields, which is fine since the run loop will reset them anyway. The run loop itself needs exact operation ordering: shuffling the node and route lists, conditionally shuffling correlatedVertices for each client index, handling empty routes by taking the smallest index from the set, and crucially calling setLocalVariablesRouteU again in the empty-route block since nodeV changed and the context may have been modified by previous moves.

The move sequence follows a specific pattern: moves 1-3, then conditional 4/5/6 based on nodeUIndex <= nodeVIndex, then 7 intra, then 8/9 inter. When nodeV is at a depot boundary, I reassign it to the depot start, recompute the intra-route flag, and try moves 1-3 again plus 8-9 if inter-route. The SWAP* block only runs when both useSwapStar is enabled and coordinates are provided. Within the correlatedVertices loop, I'm iterating through positions and filtering by whenLastModified, being careful that nodeU stays valid across moves even though the route structure changes and nodeU's route pointer gets updated.

For exportIndividual, I'm sorting routes by polar angle and recalculating costs. The printState function formats iteration counts and timing with specific widths. When exporting search progress, I'm writing the instance name, seed, penalized cost, and elapsed time using a 6-significant-digit format for the floating-point values.

For the solution file output, each route gets printed with a space before each node index, and the cost line uses the same %g formatting so test comparisons can find the cost substring exactly.

Now I'm organizing the Rust crate structure with separate modules for the core components: RNG utilities, matrix operations, algorithm parameters, the main genetic algorithm and local search logic, instance loading, and formatting helpers.

The project will live in `rust/` at the repo root, keeping it simple and conventional. For testing, I'll run against X-n101-k25 with the default termination condition of 20000 iterations without improvement.

The testing strategy is to build both the C++ reference and Rust versions, then run them on the same instances with reduced iterations for quick smoke tests, comparing costs to ensure they're in the same ballpark. The official benchmark for X-n101-k25 is 27591, which the C++ version hits with full 20k iterations in 30-60 seconds, so I'll aim for similar performance in Rust.

For performance, the distance matrix is stored flat with inline `get(i,j)` lookups using row stride, and I'm keeping bounds checks in safe code rather than using unchecked access—the overhead should be acceptable. The `Individual::new` constructor takes a mutable reference to `Params` to access the RNG for shuffling the chromosome, matching the C++ design.

I'm skipping the file-based constructor for loading solutions since it's unused by the CLI and would be dead code. For initialization, both `bestSolutionRestart` and `bestSolutionOverall` start with `penalizedCost = 1e30`, and the `getBestFound` check uses the threshold `< 1e29` to filter valid solutions. The `Population::restart` method creates new random individuals by calling `Individual::new`, which consumes RNG draws via shuffling—I need to match this RNG consumption order exactly to stay synchronized with the C++ version.

When adding individuals to the population, I'll use `clone_from` on best solutions to reuse allocations where possible, though the population itself needs to own cloned copies of individuals. For the subpopulation storage, I'm using `Vec<Box<PopIndiv>>` where each element holds an ID, the individual, biased fitness, and proximity metrics—the Box wrapper keeps pointer moves cheap when removing from the middle of the vector.

Each individual maintains a proximity list as sorted pairs of distance and ID, using binary search for insertions and `total_cmp` for safe f64 comparisons. The `averageBrokenPairsDistanceClosest` function pulls the first N entries from this sorted list, and `removeWorst` scans through to find and delete the matching proximity entry by ID. I'm replicating the tournament and biased fitness updates exactly as the C++ version does them, including the wall-clock time limit check in the generation loop and the crossover logic with its redraw conditions.

The tricky part is the borrow checker: `getBinaryTournament` mutates the population's biased fitnesses and RNG state, but I need two parent references simultaneously for crossover. I'm solving this by having the tournament selection return a lightweight handle (subpopulation marker plus index) rather than a direct reference, then retrieving the actual individuals afterward. This way each tournament call takes a mutable borrow sequentially, and the two parent borrows happen afterward as immutable references — they're disjoint from the offspring being written and from the params/RNG used during crossover.

For the handle design, I'll use an enum like `Parent { Feasible(usize), Infeasible(usize) }` to track which subpopulation and index, avoiding handle invalidation since nothing mutates the population between the two tournament calls. The local search and evaluation functions follow the same pattern with indices.

Now I'm matching the C++ output formatting — width specifiers like `{:>6}` for integers and `{:.2}` for floats — and handling the CLI arguments with the same flags and validation logic. For parsing, I'll error on invalid input rather than silently defaulting to zero, which is a reasonable deviation since the algorithm behavior stays consistent. The parameter validation messages need to replicate the C++ warnings about coordinates and fleet size, plus the safeguard error checks.

For the polarAngle calculation, I need to replicate the C++ implicit conversion where a double gets truncated to int before passing to positive_mod — so I'll cast `(32768.0 * atan2(dy, dx) / PI)` to i32, which truncates toward zero just like C++. The atan2 range stays within bounds, and edge cases like clients at the depot or exactly at angle π work out correctly. Floating-point operations between Rust and C++ may differ in the last bit due to libm variations, but that's acceptable. The round function behaves the same way, and I'm using a BTreeSet for empty routes to match the C++ std::set ordering.

Now I'm looking at the main genetic algorithm loop structure... I need to verify that `Population::printState` handles the case where one subpopulation is empty (getDiversity returns -1.0), and check the binary tournament logic—it shouldn't encounter an invalid distribution since generatePopulation guarantees at least one individual exists. For the RNG, I'm implementing Fisher-Yates shuffle and uniform sampling with rejection to ensure unbiased distribution across the range.

For the Matrix struct, I'll keep it straightforward with inline get/set methods rather than implementing Index traits. Params will own the distance matrix directly, moving it in from the instance reader to avoid copies. For the library API, I'll expose the core modules and let users construct Params directly before calling the genetic algorithm—a convenience wrapper isn't necessary.

On module naming, I'm shifting to be more idiomatic: `cvrplib.rs` with a `CvrplibInstance` struct, `cli.rs` for command-line handling, and keeping `algorithm_parameters.rs` as-is. The README will map the C++ names to Rust equivalents for clarity.

Now diving into LocalSearch, which is the most complex piece. I'm setting up the arena with client indices 0 through nb_clients, depot starts at nb_clients + 1 + r for route r, and depot ends at nb_clients + 1 + nb_vehicles + r, all stored in a single Vec<Node>. I'm mirroring the C++ structure with fields like nb_clients, nb_vehicles, search_completed, nb_moves, and order_nodes...

Defining LocalSearch fields... I'm considering using a closure to wrap the distance lookup, which would keep the formulas readable without verbose repeated calls. The closure borrows the params immutably while I mutate self independently, so there's no borrow conflict. I'm also defining inline penalty helper functions for excess duration and load violations, then setting up a function to initialize local variables for a route by extracting the node indices and positions.

Implementing the move1 function... Implementing move8 with index-based node access instead of pointers, calculating the cost delta from the 2-opt* swap and checking if the new route durations exceed penalties before committing the move. Completing the node reconnection logic...

I'm verifying the transcription handles the pointer updates correctly—specifically that when assigning `prev`, the subsequent `next` assignment reads the newly-set value. The logic checks whether nodeV can be a depot in move8, which it can when called from the depot insertion block after setting nodeV to the depot start. I'm also noting that moves 8 and 9 are only attempted in specific contexts—move8 in the main routing loop and move9 only in the empty-route case alongside moves 1, 2, and 3. Now I'm working through move9's surgery, which transfers nodes between routes by updating depot pointers and reassigning route ownership for all nodes in the affected segments. I'm setting up the `run` method to initialize penalties and load the individual, then shuffle the node and route orderings. When iterating through clients, I need to shuffle the correlated vertices for each one, but I'm running into a borrow checker issue—I can't borrow both `params.rng` and `params.correlated_vertices` mutably at the same time. The solution is to destructure the mutable reference to `params` into separate mutable borrows of its fields, so I can use `rng` and `correlated_vertices` independently without conflicts.

I'm also checking other places where this pattern appears: in move loops, methods take immutable references so there's no issue; in the main loop, `params` is used sequentially so multiple mutable borrows are fine; and in `Genetic::run`, I can pass `&mut Params` to functions that need both the RNG and population data. Now I'm moving into the main local search loop, where I track which nodes have been tested and update their last-tested timestamp.

Writing the main search loop... Continuing the move attempts in the empty-route block, then incrementing the loop counter to try again with updated state.

Now I'm implementing the SWAP* block, which iterates through pairs of routes and applies a swap operation when certain conditions are met—both routes have customers, they're in the right order, and their sectors overlap geometrically.

For the swap_star function itself, I'm setting up the core logic: preprocessing insertions for both route pairs, then iterating through nodes in each route to evaluate potential swaps. I'm calculating the penalty deltas for load imbalances when moving customers between routes, comparing costs to find the best swap configuration.

Then I'm handling the single-customer relocations — moving just customer U to route V or vice versa, using the precomputed best insertion positions and calculating the distance savings from removing the customer from its current position. Implementing the Rust version of the cheapest insert function, handling the case where best_location[0] might be None by falling back to subsequent options, and checking that the position isn't the node itself or its neighbor before considering it valid. Restructuring the borrow to avoid conflicts... Writing exportIndividual... Finishing the general_split function to set max_vehicles as the ceiling of total demand divided by vehicle capacity, matching the C++ logic. For error handling, I'm deciding between propagating Results through the call stack or using panics for internal invariant violations—user-facing errors like instance parsing go through Result and get caught in main with "EXCEPTION | ..." output, but split failures and elimination errors are pathological cases that should panic since they indicate bugs or impossible data constraints.

I'm verifying that even a single client with demand exceeding 1.5× capacity won't cause the split to fail, since the inner loop processes each client individually before checking the load bound. Now I need to implement the two versions of splitSimple—one with duration constraints using Bellman-style DP and another using a deque-based approach—making sure the predecessor tracking works correctly in both cases.

Implementing deque structure... Defining the Population struct with feasible and infeasible subpopulations, tracking feasibility constraints and search progress, plus best solutions across restarts. The PopIndiv wrapper holds each individual with its ID, biased fitness, and proximity distances to other solutions. When adding an individual, I'm computing broken-pairs distances to all existing members in the target subpopulation and maintaining sorted proximity lists for each.

I'm inserting the new individual at the correct position by penalized cost, then trimming excess members by removing the worst biased fitness scores when the subpopulation exceeds mu+lambda capacity. To avoid borrow conflicts, I'm restructuring remove_worst_biased_fitness and update_biased_fitnesses as standalone functions that operate on mutable slices rather than self, since they only need the params struct. I'm also tracking whether the new individual improves the best solution found so far.

Now I'm implementing the biased fitness update by ranking individuals based on their average broken pairs distance to closest neighbors, then assigning fitness scores that combine both fitness rank and diversity rank, with the diversity component only applying when the population exceeds the elite threshold.

I'm also defining a helper function that computes the average distance to the closest neighbors, handling the edge case where a single individual has no proximity data by returning NaN, which matches the C++ behavior and only appears cosmetically in output since the algorithm skips this calculation when population size is one.

Defining remove_worst... Now I'm implementing the penalty management by sorting infeasible individuals by their penalized cost using bubble sort, then setting up binary tournament selection that samples from both feasible and infeasible populations after updating their biased fitnesses.

For the tournament, I'm returning a tuple of (is_feasible, index) rather than a direct reference to avoid borrow checker conflicts, then resolving it later through a getter method. The real challenge is that crossover needs mutable access to the offspring and mutable params while borrowing both parents from the population — since a method call borrows all of self mutably, I'm extracting crossover as a free function that takes explicit parameters instead.

Within crossover, I can use the mutable params for RNG draws first, then pass an immutable reborrow to the split function which only needs the penalty constants. For population generation, I'm building individuals by creating random solutions, applying the split heuristic, running local search, and adding them to the population while respecting the time limit.

There's a borrow checker issue though — when calling local_search.run with params as a mutable borrow and then trying to read penalty_capacity from params in the same argument list, Rust rejects this because the read happens while params is already mutably borrowed. The fix is to extract the penalty values into local variables before the call, then pass those instead. The restart function needs to accept split and local_search references since it calls generate_population internally.

For the output formatting, I'm replicating the C++ printf statements with Rust's print! and println! macros using format specifiers like {:6} and {:.2}, and since Rust's stdout is line-buffered, the final println! handles the newline flush automatically. I also need getDiversity and getAverageCost methods that work over the first min(mu, size) individuals, plus export functions for CVRP format and search progress tracking.

Defining the Genetic struct... Continuing the main loop, I increment the iteration counter and check for penalty management and state printing at regular intervals. If we hit the time limit without improvement, I restart the population and reset the non-productive iteration counter. After the loop finishes, I print a completion message with the total iterations and elapsed time using proper formatting. I'm also noting some implementation details: the penalty capacity multiplier is evaluated at repair time and remains constant within an iteration since penalty management happens after both local search calls, so using a local copy of pc is fine. There's a question about whether the second binary tournament call invalidates the first selection handle, which I need to verify.

The crossover function implements order crossover by copying a segment from the first parent, marking those clients as used, then filling remaining positions with clients from the second parent in order, skipping any already placed. The frequency array is allocated per call to match the C++ implementation, and the index arithmetic stays within bounds since j grows at most to 3n.

Now I'm looking at the Params struct which holds algorithm configuration, problem data, and runtime state. The constructor converts from C++ with proper error handling, uses Option<usize> for the vehicle count parameter instead of INT_MAX, and calculates the default number of vehicles using the demand-based formula. For correlated vertices, I'm building the adjacency structure by sorting and deduplicating vectors to match the C++ set behavior, and the order proximity sorting pairs distances with client indices in ascending order.

For the CVRPLIB reader, I'm parsing the instance file by reading it into a string, skipping the header lines, then tokenizing the rest to extract metadata like dimension and capacity. The parsing logic mirrors the C++ approach of consuming tokens sequentially until reaching the NODE_COORD_SECTION marker, handling the colon separators and extracting values like the client count and vehicle capacity.

After the header, I'm reading the node coordinates, demands, and distance matrix, then validating the depot section format and checking for errors like invalid client numbering or undefined capacity. For the CLI, I'm building a command-line argument parser that mirrors the C++ structure with algorithm parameters, vehicle count, file paths, and flags for verbosity and integer rounding. Writing main function... Finishing the trim logic and then verifying the function against test cases: 27591.0 correctly formats to "27591", 555.43 to "555.43", and edge cases like 1e30 produce scientific notation as expected. The rounding behavior matches C++'s %g format, including cases where mantissa rounding carries over (like 999999.7 becoming 1e+06).

Now I'm defining the EvalIndiv struct to hold evaluation metrics like penalized cost, route count, distance, and capacity/duration excesses, plus a feasibility flag. I'm also noting the critical constants: MY_EPSILON at 0.00001 and PI as the specific literal 3.14159265359 to match C++'s behavior exactly, not Rust's standard library constant. For the random number generator, I'm verifying the minstd_rand implementation matches libstdc++: the state transitions via x = x * 48271 % 2147483647, seeding handles zero by setting state to 1, and the rejection sampling for uniform_below works correctly for shuffle operations.

Checking the node ordering logic — the C++ code validates nodeUIndex <= nodeVIndex after setLocalVariables is called, and I'm replicating that same sequence. The empty-route handling calls setLocalVariablesRouteU again even though it was set in the inner loop, because moves may have modified state or the correlatedVertices context might not have been initialized if there's only one client. For updateRouteData's barycenter calculation, it uses the depot coordinates from params.cli[0], and when coordinates aren't provided or useSwapStar is disabled, the barycenter becomes zero with deterministic tie-breaking by route index. I'm now considering performance implications of vector bounds checking in hot loops to ensure parity with the C++ version. Tracing through the backtracking logic: potential[k][0] for k≥1 is never read since the queue for round k starts at k, so it stays uninitialized—which is fine. The pred array doesn't need resetting either since values are only read where potential was actually computed. I'm replicating this behavior with my flat arrays, initializing pred to zeros like the C++ constructor does. The backtracking loop iterates through routes in reverse, using pred[k+1][end] to trace the path, and there's a safety check that potential[0][nbClients] must be below the sentinel value.

For the split logic: splitSimple fills chromR from the end for maxVehicles routes, and if the unlimited-fleet solution needs more vehicles than allowed, backtracking runs out of routes (end stays > 0) and returns false, triggering splitLF as a fallback. I need to handle the usize loop carefully when clearing chromR—if maxVehicles equals nbVehicles, the range is empty, which is fine. I'm clearing routes from maxVehicles onward and then backtracking through the optimal path.

Implementing the backtracking loop... Checking the move keyword — `move1` is fine in Rust. I'm also verifying the search progress export format matches the C++ output, storing elapsed time as a duration and formatting it consistently. For the algorithm parameters, I'm confirming the defaults from the C++ code: nbElite=4 and nbClose=5. Now I need to set up the Cargo.toml with the package configuration, binary and library targets, and release profile optimizations like LTO and codegen units. I'll write a README in English documenting the usage and design mapping, then outline the testing plan.

For testing, I'll build both the Rust and C++ versions and run them on standard instances like X-n101-k25 with the default 20,000 iterations to verify the final cost matches around 27591, plus test CMT6 with distance and service time constraints. I'll start with smaller smoke tests at 1,000 iterations if needed, then check instance file headers to validate parser assumptions before writing the implementation files in dependency order.

Now I'm setting up the LocalSearch structure with its node arena, where clients occupy indices 0 through nb_clients, depot nodes are positioned after that with separate start and end nodes per vehicle, and routes track their associated courier, customer count, modification timestamps, and various metrics like duration and load that get initialized to defaults and populated during individual loading.

I'm also checking the move7 operation logic — it reverses a segment between two nodes by swapping next/prev pointers along the chain, and I need to verify that position comparisons stay valid since positions only get stale across different posV iterations, but within a single iteration all previous moves returned false so no modifications occurred yet. The edge cases check out: correlated vertices can't be the same node, and the depot branch only attempts moves 1, 2, and 3 anyway.

For swapStar, the borrowing in the evaluation loop works because I'm only taking immutable references to routes and nodes while computing the penalty delta, keeping the best candidate in a local struct, and only mutating after the loops finish. The relocate blocks correctly index into the best insert position tables using flat indices for both route pairs, and even when bestPositionU lands in routeV (which is fine since insertNode handles the route reassignment), the positions remain disjoint — U is in routeU and V is in routeV, so there's no conflict. The tricky case is when the same node could theoretically end up in both routes after simultaneous insertion, but getCheapestInsertSimultRemoval correctly computes positions within the target route, so even if bestPositionV points to nodeU after it's moved, the logic still holds.

The key insight is that I'm storing position indices during evaluation before any mutations happen, then applying the insertions in order — first U, then V — which means the pointer to U->prev remains valid even after U is removed from its original route. Now I'm moving on to the Population::indiv method, which needs a clean way to handle the tournament pick parameter.

I'll define a simple struct to hold the feasibility flag and index, then use that as the return type for the binary tournament selection. This keeps the borrow checker happy since p1 and p2 are immutable borrows of the population while other fields like offspring and params are mutably borrowed separately.

For the LocalSearch shuffle operations, I'm being careful about mutable access to params — the RNG and correlated vertex structures need to be borrowed correctly when shuffling the order_nodes array.

On the type front, I'm settling on usize for counts in AlgorithmParameters, f64 for ratios and timing values, and u64 for the seed. For the use_swap_star flag, I'll keep it as a bool but print it as 1 or 0 to match the C++ output format. The timeLimit printing needs to handle the zero case properly.

For Population initialization, I'm passing a mutable reference to Params so the Individual constructors can draw from the RNG. The startup RNG consumption order matches the C++ version exactly — split, local search setup, then the three population individuals with their shuffles.

I'm tracking the RNG seeding through Params::new, confirming the instance path comes from the command line, and verifying that state printing happens at iteration multiples of nbIterTraces including iteration zero. The penalty management also triggers at multiples of 100 starting from zero, and I need to count true values in the VecDeque and replicate the penalty capacity adjustment logic precisely.

For LocalSearch, I'm implementing the loop with a while condition checking search_completed, setting it true when loop_id exceeds 1, and storing loop_id as a member since it's used in helper conditions throughout the body.

For empty routes, I'm using a BTreeSet and handling the move9 case where both nodes are depots—the surgery logic links routes by connecting the end depot of one route to the start of another, mirroring the C++ pointer manipulations exactly.

I'm verifying the depot circular structure: `myDepotFin->next = myDepot` and `myDepot->prev = myDepotFin`, so when evaluating move9 with nodeY as the end depot, accessing `nodeY->next->cour` correctly gives the start depot's courier index (0). I'm also initializing `when_last_tested_ri = -1` and `nb_moves = 0` in loadIndividual, and for ThreeBestInsert the default `when_last_calculated = 0` gets overwritten anyway, so that's fine.

Now I'm implementing the `compare_and_add` function that maintains the three best insertion costs and locations, shifting them down the ranking as better options are found. I'm considering whether to split Params into separate Instance and SearchState structs for cleaner abstraction, but that would diverge from the C++ design and add parameter threading complexity — keeping them unified in a single Params struct with mutable state threading via `&mut` is more pragmatic and maintains behavioral parity.

Checking the borrow safety: when `Population::add_individual` is called from `Genetic::run`, the population is mutable, offspring and params are immutable references — all disjoint. In `generate_population`, the random individual is local and params gets reborrows correctly. The function needs `nb_clients` from params for the broken pairs distance calculation, along with `mu`, `lambda`, and `start_time`.

The `broken_pairs_distance` function compares successors and predecessors between two individuals, counting differences and normalizing by client count. I'm verifying that successors and predecessors are always properly initialized — they're set during `evaluateCompleteCost`, and every client appears in exactly one route with depot connections handled correctly.

One edge case: `Individual::new` doesn't evaluate, leaving garbage values, but `add_individual` only accepts already-evaluated individuals, and the initial best solution with penalty cost 1e30 never gets compared via distance. For the proximity list insertion, I'm using `partition_point` with a total ordering on (distance, id) pairs — since ids are unique, even when distances match, the ordering is deterministic. The multiset behavior of inserting at the lower bound of equal keys works here because strict comparison ensures exact position consistency.

Now I need to write a README documenting the structure mapping, behavioral differences like the RNG implementation and wall-clock versus CPU timing, tie-breaking by id instead of pointer for determinism, and error handling patterns. I'm also considering whether to add a few unit tests.

I'm thinking about adding tests for known minstd values (the C++ standard specifies that the 10000th invocation from seed 1 produces 399268537), format_double edge cases, circle sector overlap, and maybe a small end-to-end test. I should also verify the CVRPLIB file format and check how Windows line endings with `\r\n` are handled — both C++ and Rust treat `\r` as whitespace, so the parser should handle them correctly.

For validation, I'm planning to run the HGS algorithm on test instances like X-n101-k25 with known optima (27591, 14971 for others) using 20000 non-improving iterations. Each iteration takes roughly 1-3 ms, so the full run should complete in 1-2 minutes. I'll also compare population traces to ensure the Rust version behaves similarly to the C++ reference.

On the output formatting side, I need to match the C++ exactly: time spent as %.2f, finished line using %g format, and "NO-FEASIBLE" printed when needed. For the local search module, a single ~700-line file mirrors the C++ structure well enough without splitting.

For visibility and API design, I'm exposing what the C++ version makes public—methods like getBestFeasible return Option<&Individual> instead of NULL pointers. The exportCVRPLibFormat function doesn't actually use the Population state, so I'll move it to individual.rs as a free function since it exports an Individual, even though C++ keeps it in Population for historical reasons.

On error handling, I'm returning Result from export functions and letting main print the exact failure message to match observable behavior, while silently ignoring errors in export_search_progress since the C++ version does the same.

Writing the RNG module... Adding matrix accessor methods and defining the circle sector and algorithm parameter structures... Now I'm planning the Population struct with an IndivRef type to identify individuals within the two subpopulations, then laying out all the methods needed: initialization, adding individuals, tournament selection, managing penalties, tracking the best solutions, and exporting search progress with timing data stored as Duration.

Writing the genetic algorithm run loop... Now I'm checking the LocalSearch::run signature and how it borrows parameters — the shared reads of `params.correlated_vertices` work fine within the loop, and the Individual::new constructor initializes the chromosome by shuffling clients and setting up the route structure with default evaluation cost.

For the Split initialization, I need to replicate the C++ loop that builds cumulative load and distance arrays — the key detail is that `cliSplit[0]` stays at default zeros since it's never explicitly written, and its distance component feeds into the first cumulative distance calculation. The tricky part is handling the boundary case where the last client gets a special sentinel value of -1e30 for distance, which affects how subsequent iterations use that value.

Now I'm implementing the propagate and dominates methods as inline functions that compute route costs using the precomputed potential, cumulative distance, and load arrays, with capacity penalty calculations built in. Writing the loop logic... Building the correlated vertices structure...

Now I'm handling the validation logic for fleet size, checking that the number of vehicles meets the minimum requirement based on total demand and capacity. I need to replicate the exact error messages from the C++ version and handle the default fleet initialization with the 1.3 multiplier plus buffer. I'm also setting up the parameter defaults—duration limit and capacity both default to a very large value, and I need to check for undefined capacity and replicate that validation message exactly.

For the CLI parsing, I'm mirroring the argument names and verbose flag logic from the original. I should add unit tests for the RNG with known minstd values, the formatting utilities, and circle sector calculations, though real validation will come from running actual instances. I'll also create a rust/.gitignore to exclude the target directory.

Now I'm implementing the binary tournament selection in the population, which draws two random individuals from the combined feasible and infeasible pools, updates their biased fitness scores, and compares them to select the winner.

Implementing helper methods... Adding individual to subpop... Writing population generation... Defining LocalSearch struct... Writing the run loop... Continuing with the move attempts on empty routes when available, then transitioning into the swap-star optimization logic that iterates through vehicle pairs to find improvements based on their cost metrics.

Now I need to implement each of the individual move functions carefully, starting with move2 and working through the rest of the move types. Finishing the cost comparison check, then validating preconditions before inserting nodes into the routes and updating their data. I'm also noting the similar structure for other move types—move3 through move6 follow comparable patterns with different swap and insertion combinations, each with their own precondition checks to prevent invalid node placements. Finishing the move7 function and then implementing a swap_node helper that exchanges two nodes' positions and routes, though I'm noticing a potential edge case when the nodes are adjacent — the self-loop issue gets resolved by the order of assignments, but it's worth verifying the logic holds in all cases. Confirmed — the guards prevent adjacent swaps in all cases ✔. Now I'm writing a function that iterates through a route starting from the depot, accumulating position, load, time, and reversal distance metrics for each node while updating their cumulative values. Restructuring the borrow scopes to avoid conflicts — I'm isolating the mutable route reference in a block so I can read `self.nb_moves` afterward, then checking if the route is empty to set its polar angle barycenter or compute it from the accumulated coordinates. Finishing the loop and moving into the load_individual function, which reconstructs the solution structure from a chromosome representation by linking nodes together into routes for each vehicle, handling both empty and populated routes. Implementing swap_star operation... Finishing the duration penalty calculation for route V, then checking if this swap move improves the best solution found so far. I'm also verifying the naming convention for the insertion cost variables matches between the C++ reference and my Rust implementation. Now moving into the relocate blocks section, where I'm iterating through clients in route V to find the best insertion position using precomputed best insert data. Checking the move cost and applying the swap if it improves the solution, then updating both routes and marking the search as incomplete. I'm also verifying the implementation details for the genetic crossover and simultaneous removal functions, and deciding to place the constants like epsilon and pi in util.rs for consistency with how they're used across the codebase. Before finalizing, I need to confirm the CMT6 file format to ensure the distance and service time columns are in the right order.

Now I'm setting up the file parsing with a CvrplibInstance struct and implementing command-line argument parsing that mirrors the C++ behavior—printing help and returning errors for unknown arguments or invalid numbers, though I'll provide clearer error messages for parse failures as an improvement. The main entry point will catch any errors and print them with the "EXCEPTION" prefix, then delegate to a run function that processes the parsed command line.

I'm building the main solver flow: reading the instance file, constructing parameters from the instance data, initializing a genetic algorithm solver, running it, and exporting the best solution found. But I've hit a borrow checker issue—I'm trying to move `instance.dist_mtx` while simultaneously borrowing other fields from the same struct in the Params constructor call.

The fix is straightforward: since the fields being borrowed are disjoint from the field being moved, Rust should allow this. For the `commandline.ap` issue, I can derive `Copy` and `Clone` on the `AlgorithmParameters` type since it only contains scalars, then pass it by value. The population borrows and best solution handling should work fine with NLL since the borrows don't overlap in problematic ways.

Now I'm writing the export functions for the search progress and CVRP format output, then setting up the README and Cargo.toml configuration for the Rust project.

I need to handle the granular shuffle carefully with the borrow split — destructuring the params to get mutable references to the RNG and correlated vertices separately so I can shuffle both without conflicts. The move4 operator also needs to maintain the same guard order as the C++ version, checking costs first before the node index conditions, even though the order doesn't affect side effects.

For the type choices, u32 works fine for crossover operations with populations up to 1000, while usize handles the total population size which stays under 130. In Individual::evaluate_complete_cost, I'm resetting the eval struct to default and accumulating costs, then computing feasibility based on whether capacity and duration excesses fall below the epsilon threshold. The penalized_cost starts at 1e30 for new individuals and gets computed properly during evaluation. Population operations use clone for adding individuals, and the tournament selection sequence avoids any borrow conflicts by getting mutable references at different times. Now I'm writing the LocalSearch penalty helper functions that compute excess duration and load penalties based on the limits and penalty coefficients.

For move8 and move9, I need to replicate the exact pruning logic: an early exit when cost >= 0 (without epsilon), then a second check after penalties with cost > -MY_EPSILON. Moves 1-6 use a different pruning condition that only applies for non-intra-route moves. The route data tracking uses i32 for when_last_modified and when_last_tested_swap_star, matching the node's when_last_tested_ri field. Node defaults derive correctly with is_depot false and all numeric fields zeroed. For instance reading, I need to remember that the C++ InstanceCVRPLIB sets durationLimit to 1 by default.

Now I'm writing the file parser. I need to skip the first three lines (NAME, COMMENT, TYPE), then tokenize the rest. I'm creating a helper function that wraps the iterator to cleanly handle missing tokens with a Result type, avoiding closure borrow conflicts.

Then I'm building the header parsing loop that reads through dimension, capacity, distance, and service time fields, extracting the number of clients and validating that required parameters like vehicle capacity are defined.

I need to handle the potential underflow when subtracting 1 from the dimension count—using saturating subtraction ensures we get 0 if n is 0, which then fails the validation check that requires n > 0, matching the C++ behavior. I'm also writing a generic parse helper that converts strings to typed values with proper error messages, then reading the node coordinates section where each node's position is validated to be in sequential order. After that comes the demand section parsing. Validating the depot section and checking for the EOF marker, then I'll write out the full Params constructor with all the initialization logic. The C++ code reads the content twice (getting "-1" then "EOF"), so I need to replicate that behavior. I'm also capturing the start time with Instant::now() to match the C++ clock() call, and I'll make sure to use usize for iteration counters consistently. After writing everything, I'll build and test both the Rust and C++ versions to compare results, keeping performance flags at their defaults and adding MIT license headers to match the original files. For the Genetic struct, I'll expose params and population as public while keeping other fields private.

Re-exporting the key types from lib.rs...

Now I need to decide on error handling for edge cases like when split.general_split has no solution — I'll keep that as a panic since it's an invariant violation. Same with removing the worst individual. For Population::get_binary_tournament, I should add an assert that total > 0 to avoid undefined behavior in the uniform sampling. I also need to replicate the exact comparison logic in addIndividual where equal costs shift the insertion point left, and handle the edge case in update_biased_fitnesses where an empty population would skip the ranking entirely.

I'll add a clippy allow attribute for too_many_arguments on Params::new since it's unavoidable, then move into writing the LocalSearch file. Need to be careful with the move operations — move5 does a swap then insert, move6 does two swaps, and moves 1-3 have specific preconditions around depot checks and node equality that I need to replicate exactly.

For the penalty calculations, the key thing is that when computing costSuppU and costSuppV, I'm using the raw distance deltas before penalties are applied, then adding penalties based on those raw values — the order matters because the penalty function receives the raw cost inside the calculation. Same pattern for both routes.

I'll make all Params fields public since LocalSearch, Split, and Population need to access them frequently, keep Population fields private with public methods, and do the same for LocalSearch with public run/load/export methods.

Now laying out the complete file structure and writing everything out.

</details>

源码已全部理解。现在我来检查一下实例文件格式,确认解析器的假设:

<details>
<summary>Tool Call: head -12 Instances/CVRP/X-n101-k25.vrp; echo ===; head -10 Instances/CVRP/CMT6.vrp; echo ===; tail -5 Instances/CVRP/X-n101-k25.vrp</summary>

*Status: Completed*

Terminal:
```
NAME : 	X-n101-k25	
COMMENT : 	"Generated by Uchoa, Pecin, Pessoa, Poggi, Subramanian, and Vidal (2013)"	
TYPE : 	CVRP	
DIMENSION : 	101	
EDGE_WEIGHT_TYPE : 	EUC_2D	
CAPACITY : 	206	
NODE_COORD_SECTION		
1	365	689
2	146	180
3	792	5
4	658	510
5	461	270
===
NAME : CMT6
COMMENT : 555.43
TYPE : CVRP
DIMENSION : 51
EDGE_WEIGHT_TYPE : EUC_2D
CAPACITY : 160
DISTANCE : 200.00000
SERVICE_TIME : 10.0
NODE_COORD_SECTION
1 30.00000 40.00000
===
101	35	
DEPOT_SECTION		
	1	
	-1	
EOF		

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

I'll set up the Rust project structure and start building out the foundational modules.格式确认无误，现在开始创建 Rust 项目的脚手架和基础模块。

</details>

<details>
<summary>Tool Call: Create directory \`HGS\-CVRP/rust/src\`</summary>

*Status: Completed*

Created directory HGS-CVRP/rust/src

</details>
<details>
<summary>Tool Call: rust/Cargo.toml</summary>

*Status: Failed*

Can't create file: parent directory doesn't exist

</details>
## Assistant

<details>
<summary>思考过程</summary>

The directory creation succeeded but the write operation failed, possibly due to a timing issue—I should retry the write operation.目录刚创建可能有延迟，我现在重试写入。

</details>

<details>
<summary>Tool Call: rust/Cargo.toml</summary>

*Status: Completed*

Diff: rust/Cargo.toml
```
[package]
name = "hgs-cvrp"
version = "1.0.0"
edition = "2021"
description = "Rust implementation of the Hybrid Genetic Search for the CVRP (HGS-CVRP)"
license = "MIT"

[lib]
name = "hgs_cvrp"
path = "src/lib.rs"

[[bin]]
name = "hgs"
path = "src/main.rs"

[profile.release]
lto = true
codegen-units = 1

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

I'll start building out the core source files, beginning with the utility functions, random number generation, matrix operations, circle sector calculations, and algorithm parameters.现在我来写基础模块，包括 `util.rs`、`rng.rs`、`matrix.rs` 和 `circle_sector.rs`。

</details>

<details>
<summary>Tool Call: rust/src/util.rs</summary>

*Status: Completed*

Diff: rust/src/util.rs
```
//! Small shared utilities and constants.

/// Precision parameter, used to avoid numerical instabilities (same as the C++ MY_EPSILON).
pub const MY_EPSILON: f64 = 0.00001;

/// Pi constant, kept identical to the C++ implementation for bit-compatible polar angles.
pub const PI: f64 = 3.14159265359;

/// Formats a float like the C++ default `std::ostream` (printf "%g" with 6 significant digits).
///
/// This keeps solution files and logs textually identical to the reference implementation.
pub fn format_double(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    if value.is_nan() {
        return "nan".to_string();
    }
    if value.is_infinite() {
        return if value < 0.0 { "-inf" } else { "inf" }.to_string();
    }

    // Round to 6 significant digits first, then decide between fixed and scientific notation.
    let scientific = format!("{:.5e}", value);
    let (mantissa, exponent) = scientific
        .split_once('e')
        .expect("e-notation always has an exponent");
    let exponent: i32 = exponent.parse().expect("exponent is a valid integer");

    if exponent < -4 || exponent >= 6 {
        let mantissa = mantissa.trim_end_matches('0').trim_end_matches('.');
        let sign = if exponent < 0 { '-' } else { '+' };
        format!("{}e{}{:02}", mantissa, sign, exponent.abs())
    } else {
        let decimals = (5 - exponent).max(0) as usize;
        let fixed = format!("{:.*}", decimals, value);
        if fixed.contains('.') {
            fixed
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        } else {
            fixed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::format_double;

    #[test]
    fn formats_like_cpp_ostream() {
        assert_eq!(format_double(0.0), "0");
        assert_eq!(format_double(27591.0), "27591");
        assert_eq!(format_double(555.43), "555.43");
        assert_eq!(format_double(909.675), "909.675");
        assert_eq!(format_double(1077.5499999999999), "1077.55");
        assert_eq!(format_double(0.5), "0.5");
        assert_eq!(format_double(0.2), "0.2");
        assert_eq!(format_double(1e30), "1e+30");
        assert_eq!(format_double(999999.7), "1e+06");
        assert_eq!(format_double(123456.0), "123456");
        assert_eq!(format_double(-555.43), "-555.43");
        assert_eq!(format_double(0.000012345678), "1.23457e-05");
    }
}

```

</details>
<details>
<summary>Tool Call: rust/src/rng.rs</summary>

*Status: Completed*

Diff: rust/src/rng.rs
```
//! Random number generation.
//!
//! The C++ implementation relies on `std::minstd_rand`: the algorithm only needs fast,
//! reasonably distributed random numbers, the generator quality is not critical.
//! The same LCG engine is reproduced here. The derived helpers (`uniform_below`, `shuffle`)
//! are distributionally equivalent to `std::uniform_int_distribution` and `std::shuffle`,
//! but not draw-for-draw identical to a specific C++ standard library.

/// Linear congruential generator with the parameters of C++ `std::minstd_rand`
/// (Park-Miller: `x[n+1] = x[n] * 48271 mod (2^31 - 1)`).
pub struct MinstdRand {
    state: u32,
}

impl MinstdRand {
    const MODULUS: u64 = 2_147_483_647; // 2^31 - 1
    const MULTIPLIER: u64 = 48_271;

    /// Seeds the generator like `std::minstd_rand::seed`: a seed mapping to state 0 becomes 1.
    pub fn new(seed: u64) -> Self {
        let state = (seed % Self::MODULUS) as u32;
        Self {
            state: if state == 0 { 1 } else { state },
        }
    }

    /// Returns the next raw value, uniformly distributed in [1, 2^31 - 2].
    #[inline]
    pub fn next(&mut self) -> u32 {
        self.state = ((self.state as u64 * Self::MULTIPLIER) % Self::MODULUS) as u32;
        self.state
    }

    /// Returns a uniform value in [0, n). Rejection sampling avoids modulo bias.
    #[inline]
    pub fn uniform_below(&mut self, n: u32) -> u32 {
        debug_assert!(n >= 1);
        let range = (Self::MODULUS - 1) as u32; // next() yields `range` distinct values
        let limit = range - range % n;
        loop {
            let value = self.next() - 1; // uniform in [0, range)
            if value < limit {
                return value % n;
            }
        }
    }

    /// Fisher-Yates shuffle driven by this generator.
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = self.uniform_below(i as u32 + 1) as usize;
            slice.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MinstdRand;

    #[test]
    fn matches_cpp_minstd_rand() {
        // First values of std::minstd_rand seeded with 1.
        let mut rng = MinstdRand::new(1);
        assert_eq!(rng.next(), 48_271);
        assert_eq!(rng.next(), 182_605_794);

        // The C++ standard states that the 10000th consecutive invocation
        // of a default-constructed std::minstd_rand produces 399268537.
        let mut rng = MinstdRand::new(1);
        let mut value = 0;
        for _ in 0..10_000 {
            value = rng.next();
        }
        assert_eq!(value, 399_268_537);
    }

    #[test]
    fn zero_seed_maps_to_one() {
        let mut a = MinstdRand::new(0);
        let mut b = MinstdRand::new(1);
        assert_eq!(a.next(), b.next());
    }

    #[test]
    fn uniform_below_is_in_range() {
        let mut rng = MinstdRand::new(42);
        for n in 1..50 {
            for _ in 0..100 {
                assert!(rng.uniform_below(n) < n);
            }
        }
    }
}

```

</details>
<details>
<summary>Tool Call: rust/src/matrix.rs</summary>

*Status: Completed*

Diff: rust/src/matrix.rs
```
//! Dense square matrix used for the distance/time matrix.
//!
//! Stored as a flat row-major `Vec<f64>` rather than nested vectors for better
//! cache locality, since the local search reads it in tight loops.

#[derive(Clone)]
pub struct SquareMatrix {
    size: usize,
    data: Vec<f64>,
}

impl SquareMatrix {
    pub fn new(size: usize, value: f64) -> Self {
        Self {
            size,
            data: vec![value; size * size],
        }
    }

    #[inline]
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i * self.size + j]
    }

    #[inline]
    pub fn set(&mut self, i: usize, j: usize, value: f64) {
        self.data[i * self.size + j] = value;
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

```

</details>
<details>
<summary>Tool Call: rust/src/circle\_sector.rs</summary>

*Status: Completed*

Diff: rust/src/circle_sector.rs
```
//! Simple data structure to represent circle sectors.
//!
//! Angles are measured in [0, 65535] instead of [0, 359] so that modulo operations
//! are much faster (since 2^16 = 65536).
//! Credit to Fabian Giesen ("Intervals in modular arithmetic") for implementation
//! tips regarding interval overlaps in modular arithmetics.

#[derive(Clone, Copy, Default)]
pub struct CircleSector {
    pub start: i32,
    pub end: i32,
}

impl CircleSector {
    /// Positive modulo 65536.
    #[inline]
    pub fn positive_mod(i: i32) -> i32 {
        // 1) Using the formula positive_mod(n, x) = (n % x + x) % x
        // 2) "n % 65536" is compiled as "n & 0xffff" for faster calculations
        (i % 65536 + 65536) % 65536
    }

    /// Initializes a circle sector from a single point.
    pub fn initialize(&mut self, point: i32) {
        self.start = point;
        self.end = point;
    }

    /// Tests if a point is enclosed in the circle sector.
    pub fn is_enclosed(&self, point: i32) -> bool {
        Self::positive_mod(point - self.start) <= Self::positive_mod(self.end - self.start)
    }

    /// Tests overlap of two circle sectors.
    pub fn overlap(sector1: &CircleSector, sector2: &CircleSector) -> bool {
        Self::positive_mod(sector2.start - sector1.start)
            <= Self::positive_mod(sector1.end - sector1.start)
            || Self::positive_mod(sector1.start - sector2.start)
                <= Self::positive_mod(sector2.end - sector2.start)
    }

    /// Extends the circle sector to include an additional point.
    /// Done in a "greedy" way, such that the resulting circle sector is the smallest.
    pub fn extend(&mut self, point: i32) {
        if !self.is_enclosed(point) {
            if Self::positive_mod(point - self.end) <= Self::positive_mod(self.start - point) {
                self.end = point;
            } else {
                self.start = point;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CircleSector;

    #[test]
    fn enclosure_and_overlap() {
        let mut sector = CircleSector::default();
        sector.initialize(1000);
        assert!(sector.is_enclosed(1000));
        sector.extend(2000);
        assert!(sector.is_enclosed(1500));
        assert!(!sector.is_enclosed(3000));

        // Sector wrapping around 0.
        let mut wrapping = CircleSector::default();
        wrapping.initialize(65000);
        wrapping.extend(500);
        assert!(wrapping.is_enclosed(0));
        assert!(!wrapping.is_enclosed(30000));

        assert!(CircleSector::overlap(&sector, &sector));
        assert!(!CircleSector::overlap(&sector, &wrapping));
    }
}

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

Now I'm moving on to the Rust files for algorithm parameters, general parameters, and the individual representation.

</details>

现在写 `algorithm_parameters.rs` 和 `params.rs`:

<details>
<summary>Tool Call: rust/src/algorithm\_parameters.rs</summary>

*Status: Completed*

Diff: rust/src/algorithm_parameters.rs
```
//! User-tunable parameters of the HGS algorithm.

use crate::util::format_double;

#[derive(Clone, Copy)]
pub struct AlgorithmParameters {
    /// Granular search parameter, limits the number of moves in the RI local search.
    pub nb_granular: usize,
    /// Minimum population size.
    pub mu: usize,
    /// Number of solutions created before reaching the maximum population size (i.e., generation size).
    pub lambda: usize,
    /// Number of elite individuals.
    pub nb_elite: usize,
    /// Number of closest solutions/individuals considered when calculating diversity contribution.
    pub nb_close: usize,

    /// Number of iterations between penalty updates.
    pub nb_iter_penalty_management: usize,
    /// Reference proportion of feasible individuals, used for the adaptation of the penalty parameters.
    pub target_feasible: f64,
    /// Multiplier used to decrease penalty parameters if there are sufficient feasible individuals.
    pub penalty_decrease: f64,
    /// Multiplier used to increase penalty parameters if there are insufficient feasible individuals.
    pub penalty_increase: f64,

    /// Random seed.
    pub seed: u64,
    /// Number of iterations without improvement until termination (or restart if a time limit is given).
    pub nb_iter: usize,
    /// Number of iterations between traces display during HGS execution.
    pub nb_iter_traces: usize,
    /// CPU time limit until termination in seconds. 0 means inactive.
    pub time_limit: f64,
    /// Use the SWAP* local search or not. Only available when coordinates are provided.
    pub use_swap_star: bool,
}

impl Default for AlgorithmParameters {
    fn default() -> Self {
        Self {
            nb_granular: 20,
            mu: 25,
            lambda: 40,
            nb_elite: 4,
            nb_close: 5,
            nb_iter_penalty_management: 100,
            target_feasible: 0.2,
            penalty_decrease: 0.85,
            penalty_increase: 1.2,
            seed: 0,
            nb_iter: 20_000,
            nb_iter_traces: 500,
            time_limit: 0.0,
            use_swap_star: true,
        }
    }
}

impl AlgorithmParameters {
    /// Prints all parameter values, mirroring the C++ `print_algorithm_parameters`.
    pub fn print(&self) {
        println!("=========== Algorithm Parameters =================");
        println!(
            "---- nbGranular              is set to {}",
            self.nb_granular
        );
        println!("---- mu                      is set to {}", self.mu);
        println!("---- lambda                  is set to {}", self.lambda);
        println!("---- nbElite                 is set to {}", self.nb_elite);
        println!("---- nbClose                 is set to {}", self.nb_close);
        println!(
            "---- nbIterPenaltyManagement is set to {}",
            self.nb_iter_penalty_management
        );
        println!(
            "---- targetFeasible          is set to {}",
            format_double(self.target_feasible)
        );
        println!(
            "---- penaltyDecrease         is set to {}",
            format_double(self.penalty_decrease)
        );
        println!(
            "---- penaltyIncrease         is set to {}",
            format_double(self.penalty_increase)
        );
        println!("---- seed                    is set to {}", self.seed);
        println!("---- nbIter                  is set to {}", self.nb_iter);
        println!(
            "---- nbIterTraces            is set to {}",
            self.nb_iter_traces
        );
        println!(
            "---- timeLimit               is set to {}",
            format_double(self.time_limit)
        );
        println!(
            "---- useSwapStar             is set to {}",
            self.use_swap_star as i32
        );
        println!("==================================================");
    }
}

```

</details>
<details>
<summary>Tool Call: rust/src/params.rs</summary>

*Status: Completed*

Diff: rust/src/params.rs
```
//! Main data structures for the method: instance data, penalties and RNG state.

use std::time::Instant;

use crate::algorithm_parameters::AlgorithmParameters;
use crate::circle_sector::CircleSector;
use crate::matrix::SquareMatrix;
use crate::rng::MinstdRand;
use crate::util::PI;

#[derive(Clone, Default)]
pub struct Client {
    /// Coordinate X.
    pub coord_x: f64,
    /// Coordinate Y.
    pub coord_y: f64,
    /// Service duration.
    pub service_duration: f64,
    /// Demand.
    pub demand: f64,
    /// Polar angle of the client around the depot, measured in degrees and truncated for convenience.
    pub polar_angle: i32,
}

/// Stores the problem data along with the mutable search state (adaptive penalties, RNG).
///
/// Like in the C++ implementation, the penalties and the RNG live here so that all the
/// components share a single source of truth; in Rust this means a `&mut Params` is
/// threaded through the calls that update them.
pub struct Params {
    /* PARAMETERS OF THE GENETIC ALGORITHM */
    pub verbose: bool,
    pub ap: AlgorithmParameters,

    /* ADAPTIVE PENALTY COEFFICIENTS */
    pub penalty_capacity: f64,
    pub penalty_duration: f64,

    /* START TIME OF THE ALGORITHM (wall clock, the C++ version uses CPU clock) */
    pub start_time: Instant,

    /* RANDOM NUMBER GENERATOR */
    pub rng: MinstdRand,

    /* DATA OF THE PROBLEM INSTANCE */
    pub is_duration_constraint: bool,
    pub nb_clients: usize,
    pub nb_vehicles: usize,
    pub duration_limit: f64,
    pub vehicle_capacity: f64,
    pub total_demand: f64,
    pub max_demand: f64,
    pub max_dist: f64,
    pub clients: Vec<Client>,
    /// Distance matrix.
    pub time_cost: SquareMatrix,
    /// Neighborhood restrictions: for each client, list of nearby customers.
    pub correlated_vertices: Vec<Vec<usize>>,
    pub are_coordinates_provided: bool,
}

impl Params {
    /// Builds the parameters from a given data set.
    ///
    /// `nb_veh` set to `None` triggers a default fleet size initialization
    /// (the C++ version uses INT_MAX for the same purpose).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        x_coords: &[f64],
        y_coords: &[f64],
        dist_mtx: SquareMatrix,
        service_time: &[f64],
        demands: &[f64],
        vehicle_capacity: f64,
        duration_limit: f64,
        nb_veh: Option<usize>,
        is_duration_constraint: bool,
        verbose: bool,
        ap: AlgorithmParameters,
    ) -> Result<Params, String> {
        // This marks the starting time of the algorithm
        let start_time = Instant::now();

        let nb_clients = demands.len() - 1; // Need to subtract the depot from the number of nodes
        let rng = MinstdRand::new(ap.seed);

        // Check if valid coordinates are provided
        let are_coordinates_provided =
            demands.len() == x_coords.len() && demands.len() == y_coords.len();

        let mut clients = vec![Client::default(); nb_clients + 1];
        let mut total_demand = 0.0;
        let mut max_demand = 0.0;
        for i in 0..=nb_clients {
            // If use_swap_star is false, x_coords and y_coords may be empty.
            if ap.use_swap_star && are_coordinates_provided {
                clients[i].coord_x = x_coords[i];
                clients[i].coord_y = y_coords[i];
                clients[i].polar_angle = CircleSector::positive_mod(
                    (32768.0
                        * (clients[i].coord_y - y_coords[0])
                            .atan2(clients[i].coord_x - x_coords[0])
                        / PI) as i32,
                );
            }
            clients[i].service_duration = service_time[i];
            clients[i].demand = demands[i];
            if clients[i].demand > max_demand {
                max_demand = clients[i].demand;
            }
            total_demand += clients[i].demand;
        }

        if verbose && ap.use_swap_star && !are_coordinates_provided {
            println!("----- NO COORDINATES HAVE BEEN PROVIDED, SWAP* NEIGHBORHOOD WILL BE DEACTIVATED BY DEFAULT");
        }

        // Default initialization if the number of vehicles has not been provided by the user
        let nb_vehicles = match nb_veh {
            None => {
                // Safety margin: 30% + 3 more vehicles than the trivial bin packing LB
                let default_veh = (1.3 * total_demand / vehicle_capacity).ceil() as usize + 3;
                if verbose {
                    println!(
                        "----- FLEET SIZE WAS NOT SPECIFIED: DEFAULT INITIALIZATION TO {} VEHICLES",
                        default_veh
                    );
                }
                default_veh
            }
            Some(n) => {
                if verbose {
                    println!("----- FLEET SIZE SPECIFIED: SET TO {} VEHICLES", n);
                }
                n
            }
        };

        // Calculation of the maximum distance
        let mut max_dist = 0.0;
        for i in 0..=nb_clients {
            for j in 0..=nb_clients {
                if dist_mtx.get(i, j) > max_dist {
                    max_dist = dist_mtx.get(i, j);
                }
            }
        }

        // Calculation of the correlated vertices for each customer (for the granular restriction)
        let mut set_correlated_vertices: Vec<Vec<usize>> = vec![Vec::new(); nb_clients + 1];
        let mut order_proximity: Vec<(f64, usize)> = Vec::new();
        for i in 1..=nb_clients {
            order_proximity.clear();
            for j in 1..=nb_clients {
                if i != j {
                    order_proximity.push((dist_mtx.get(i, j), j));
                }
            }
            order_proximity.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));

            for &(_, j) in order_proximity
                .iter()
                .take(ap.nb_granular.min(nb_clients - 1))
            {
                // If i is correlated with j, then j should be correlated with i
                set_correlated_vertices[i].push(j);
                set_correlated_vertices[j].push(i);
            }
        }

        // Filling the vector of correlated vertices (sorted unique values, as the C++ std::set)
        let mut correlated_vertices: Vec<Vec<usize>> = vec![Vec::new(); nb_clients + 1];
        for i in 1..=nb_clients {
            let mut vertices = std::mem::take(&mut set_correlated_vertices[i]);
            vertices.sort_unstable();
            vertices.dedup();
            correlated_vertices[i] = vertices;
        }

        // Safeguards to avoid possible numerical instability in case of instances
        // containing arbitrarily small or large numerical values
        if max_dist < 0.1 || max_dist > 100000.0 {
            return Err("The distances are of very small or large scale. This could impact numerical stability. Please rescale the dataset and run again.".to_string());
        }
        if max_demand < 0.1 || max_demand > 100000.0 {
            return Err("The demand quantities are of very small or large scale. This could impact numerical stability. Please rescale the dataset and run again.".to_string());
        }
        if (nb_vehicles as f64) < (total_demand / vehicle_capacity).ceil() {
            return Err(
                "Fleet size is insufficient to service the considered clients.".to_string(),
            );
        }

        // A reasonable scale for the initial values of the penalties
        let penalty_duration = 1.0;
        let penalty_capacity = (max_dist / max_demand).clamp(0.1, 1000.0);

        if verbose {
            println!(
                "----- INSTANCE SUCCESSFULLY LOADED WITH {} CLIENTS AND {} VEHICLES",
                nb_clients, nb_vehicles
            );
        }

        Ok(Params {
            verbose,
            ap,
            penalty_capacity,
            penalty_duration,
            start_time,
            rng,
            is_duration_constraint,
            nb_clients,
            nb_vehicles,
            duration_limit,
            vehicle_capacity,
            total_demand,
            max_demand,
            max_dist,
            clients,
            time_cost: dist_mtx,
            correlated_vertices,
            are_coordinates_provided,
        })
    }
}

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

I'm verifying that the C++ clamping operation works correctly, then moving on to refactor the individual and split modules.

</details>

现在写 `individual.rs` 和 `split.rs`:

<details>
<summary>Tool Call: rust/src/individual.rs</summary>

*Status: Completed*

Diff: rust/src/individual.rs
```
//! Representation of an individual solution in the genetic algorithm.

use crate::params::Params;
use crate::util::{format_double, MY_EPSILON};

#[derive(Clone, Default)]
pub struct EvalIndiv {
    /// Penalized cost of the solution.
    pub penalized_cost: f64,
    /// Number of routes.
    pub nb_routes: usize,
    /// Total distance.
    pub distance: f64,
    /// Sum of excess load in all routes.
    pub capacity_excess: f64,
    /// Sum of excess duration in all routes.
    pub duration_excess: f64,
    /// Feasibility status of the individual.
    pub is_feasible: bool,
}

#[derive(Clone)]
pub struct Individual {
    /// Solution cost parameters.
    pub eval: EvalIndiv,
    /// Giant tour representing the individual.
    pub chrom_t: Vec<usize>,
    /// For each vehicle, the associated sequence of deliveries (complete solution).
    pub chrom_r: Vec<Vec<usize>>,
    /// For each node, the successor in the solution (can be the depot 0).
    pub successors: Vec<usize>,
    /// For each node, the predecessor in the solution (can be the depot 0).
    pub predecessors: Vec<usize>,
}

impl Individual {
    /// Creates a random individual containing only a giant tour with a shuffled visit order.
    pub fn new(params: &mut Params) -> Self {
        let mut chrom_t: Vec<usize> = (1..=params.nb_clients).collect();
        params.rng.shuffle(&mut chrom_t);
        Self {
            eval: EvalIndiv {
                penalized_cost: 1.0e30,
                ..EvalIndiv::default()
            },
            chrom_t,
            chrom_r: vec![Vec::new(); params.nb_vehicles],
            successors: vec![0; params.nb_clients + 1],
            predecessors: vec![0; params.nb_clients + 1],
        }
    }

    /// Measures cost and feasibility of the individual from the information of chrom_r.
    pub fn evaluate_complete_cost(&mut self, params: &Params) {
        self.eval = EvalIndiv::default();
        for route in &self.chrom_r {
            if route.is_empty() {
                continue;
            }
            let mut distance = params.time_cost.get(0, route[0]);
            let mut load = params.clients[route[0]].demand;
            let mut service = params.clients[route[0]].service_duration;
            self.predecessors[route[0]] = 0;
            for i in 1..route.len() {
                distance += params.time_cost.get(route[i - 1], route[i]);
                load += params.clients[route[i]].demand;
                service += params.clients[route[i]].service_duration;
                self.predecessors[route[i]] = route[i - 1];
                self.successors[route[i - 1]] = route[i];
            }
            self.successors[route[route.len() - 1]] = 0;
            distance += params.time_cost.get(route[route.len() - 1], 0);
            self.eval.distance += distance;
            self.eval.nb_routes += 1;
            if load > params.vehicle_capacity {
                self.eval.capacity_excess += load - params.vehicle_capacity;
            }
            if distance + service > params.duration_limit {
                self.eval.duration_excess += distance + service - params.duration_limit;
            }
        }

        self.eval.penalized_cost = self.eval.distance
            + self.eval.capacity_excess * params.penalty_capacity
            + self.eval.duration_excess * params.penalty_duration;
        self.eval.is_feasible =
            self.eval.capacity_excess < MY_EPSILON && self.eval.duration_excess < MY_EPSILON;
    }
}

/// Exports an individual to a file in CVRPLib format.
pub fn export_cvrplib_format(indiv: &Individual, path: &str) -> std::io::Result<()> {
    let mut output = String::new();
    for (k, route) in indiv.chrom_r.iter().enumerate() {
        if !route.is_empty() {
            // Route IDs start at 1 in the file format
            output.push_str(&format!("Route #{}:", k + 1));
            for &i in route {
                output.push_str(&format!(" {}", i));
            }
            output.push('\n');
        }
    }
    output.push_str(&format!(
        "Cost {}\n",
        format_double(indiv.eval.penalized_cost)
    ));
    std::fs::write(path, output)
}

```

</details>
<details>
<summary>Tool Call: rust/src/split.rs</summary>

*Status: Completed*

Diff: rust/src/split.rs
```
//! Algorithms to decode solutions represented as giant tours into complete CVRP solutions.
//!
//! Direct port of the C++ implementation, itself based on the linear Split of
//! "Vidal, T. (2016). Split algorithm in O(n) for the capacitated vehicle routing problem".

use crate::individual::Individual;
use crate::params::Params;
use crate::util::MY_EPSILON;

#[derive(Clone, Default)]
struct ClientSplit {
    demand: f64,
    service_time: f64,
    d0_x: f64,
    dx_0: f64,
    dnext: f64,
}

/// Simple deque used by the linear Split algorithms.
/// Cursors are signed so that the back can transiently move before the front.
struct TrivialDeque {
    elements: Vec<usize>,
    index_front: i32,
    index_back: i32,
}

impl TrivialDeque {
    fn new(nb_elements: usize, first_node: usize) -> Self {
        let mut elements = vec![0; nb_elements];
        elements[0] = first_node;
        Self {
            elements,
            index_front: 0,
            index_back: 0,
        }
    }

    #[inline]
    fn pop_front(&mut self) {
        self.index_front += 1;
    }

    #[inline]
    fn pop_back(&mut self) {
        self.index_back -= 1;
    }

    #[inline]
    fn push_back(&mut self, i: usize) {
        self.index_back += 1;
        self.elements[self.index_back as usize] = i;
    }

    #[inline]
    fn get_front(&self) -> usize {
        self.elements[self.index_front as usize]
    }

    #[inline]
    fn get_next_front(&self) -> usize {
        self.elements[(self.index_front + 1) as usize]
    }

    #[inline]
    fn get_back(&self) -> usize {
        self.elements[self.index_back as usize]
    }

    fn reset(&mut self, first_node: usize) {
        self.elements[0] = first_node;
        self.index_back = 0;
        self.index_front = 0;
    }

    #[inline]
    fn size(&self) -> i32 {
        self.index_back - self.index_front + 1
    }
}

pub struct Split {
    nb_clients: usize,
    nb_vehicles: usize,
    max_vehicles: usize,

    /* Auxiliary data structures to run the Linear Split algorithm */
    cli_split: Vec<ClientSplit>,
    /// Potential vector, flat (nb_vehicles + 1) x (nb_clients + 1).
    potential: Vec<f64>,
    /// Index of the predecessor in an optimal path, same layout as `potential`.
    pred: Vec<usize>,
    /// sum_distance[i] for i > 1 contains the sum of distances: sum_{k=1}^{i-1} d_{k,k+1}.
    sum_distance: Vec<f64>,
    /// sum_load[i] for i >= 1 contains the sum of loads: sum_{k=1}^{i} q_k.
    sum_load: Vec<f64>,
    /// sum_service[i] for i >= 1 contains the sum of service time: sum_{k=1}^{i} s_k.
    sum_service: Vec<f64>,
}

impl Split {
    pub fn new(params: &Params) -> Self {
        let nb_clients = params.nb_clients;
        let nb_vehicles = params.nb_vehicles;
        Self {
            nb_clients,
            nb_vehicles,
            max_vehicles: 0,
            cli_split: vec![ClientSplit::default(); nb_clients + 1],
            potential: vec![1.0e30; (nb_vehicles + 1) * (nb_clients + 1)],
            pred: vec![0; (nb_vehicles + 1) * (nb_clients + 1)],
            sum_distance: vec![0.0; nb_clients + 1],
            sum_load: vec![0.0; nb_clients + 1],
            sum_service: vec![0.0; nb_clients + 1],
        }
    }

    #[inline]
    fn idx(&self, k: usize, i: usize) -> usize {
        k * (self.nb_clients + 1) + i
    }

    /// Computes the cost of propagating the label i until j (to be called with i < j only).
    #[inline]
    fn propagate(&self, i: usize, j: usize, k: usize, params: &Params) -> f64 {
        self.potential[self.idx(k, i)] + self.sum_distance[j] - self.sum_distance[i + 1]
            + self.cli_split[i + 1].d0_x
            + self.cli_split[j].dx_0
            + params.penalty_capacity
                * (self.sum_load[j] - self.sum_load[i] - params.vehicle_capacity).max(0.0)
    }

    /// Tests if i dominates j as a predecessor for all nodes x >= j + 1 (assuming i < j).
    #[inline]
    fn dominates(&self, i: usize, j: usize, k: usize, params: &Params) -> bool {
        self.potential[self.idx(k, j)] + self.cli_split[j + 1].d0_x
            > self.potential[self.idx(k, i)] + self.cli_split[i + 1].d0_x + self.sum_distance[j + 1]
                - self.sum_distance[i + 1]
                + params.penalty_capacity * (self.sum_load[j] - self.sum_load[i])
    }

    /// Tests if j dominates i as a predecessor for all nodes x >= j + 1 (assuming i < j).
    #[inline]
    fn dominates_right(&self, i: usize, j: usize, k: usize) -> bool {
        self.potential[self.idx(k, j)] + self.cli_split[j + 1].d0_x
            < self.potential[self.idx(k, i)] + self.cli_split[i + 1].d0_x + self.sum_distance[j + 1]
                - self.sum_distance[i + 1]
                + MY_EPSILON
    }

    /// General Split function: tests the unlimited fleet Split first, and only if it
    /// does not produce a feasible solution, runs the Split algorithm for a limited fleet.
    pub fn general_split(
        &mut self,
        params: &Params,
        indiv: &mut Individual,
        nb_max_vehicles: usize,
    ) {
        // Do not apply Split with fewer vehicles than the trivial (LP) bin packing bound
        self.max_vehicles =
            nb_max_vehicles.max((params.total_demand / params.vehicle_capacity).ceil() as usize);

        // Initialization of the data structures for the linear split algorithms
        for i in 1..=params.nb_clients {
            let client = indiv.chrom_t[i - 1];
            self.cli_split[i].demand = params.clients[client].demand;
            self.cli_split[i].service_time = params.clients[client].service_duration;
            self.cli_split[i].d0_x = params.time_cost.get(0, client);
            self.cli_split[i].dx_0 = params.time_cost.get(client, 0);
            self.cli_split[i].dnext = if i < params.nb_clients {
                params.time_cost.get(client, indiv.chrom_t[i])
            } else {
                -1.0e30
            };
            self.sum_load[i] = self.sum_load[i - 1] + self.cli_split[i].demand;
            self.sum_service[i] = self.sum_service[i - 1] + self.cli_split[i].service_time;
            self.sum_distance[i] = self.sum_distance[i - 1] + self.cli_split[i - 1].dnext;
        }

        // We first try the simple split, and then the Split with limited fleet if not successful
        if !self.split_simple(params, indiv) {
            self.split_lf(params, indiv);
        }

        // Build up the rest of the Individual structure
        indiv.evaluate_complete_cost(params);
    }

    /// Split for unlimited fleet. Returns true if the algorithm reached the beginning of the routes.
    fn split_simple(&mut self, params: &Params, indiv: &mut Individual) -> bool {
        // Reinitialize the potential structure
        self.potential[self.idx(0, 0)] = 0.0;
        for i in 1..=params.nb_clients {
            self.potential[self.idx(0, i)] = 1.0e30;
        }

        // MAIN ALGORITHM -- Simple Split using Bellman's algorithm in topological order.
        // This code has been maintained as it is very simple and can be easily adapted to
        // a variety of constraints, whereas the O(n) Split has a more restricted scope.
        if params.is_duration_constraint {
            for i in 0..params.nb_clients {
                let mut load = 0.0;
                let mut distance = 0.0;
                let mut service_duration = 0.0;
                let mut j = i + 1;
                while j <= params.nb_clients && load <= 1.5 * params.vehicle_capacity {
                    load += self.cli_split[j].demand;
                    service_duration += self.cli_split[j].service_time;
                    if j == i + 1 {
                        distance += self.cli_split[j].d0_x;
                    } else {
                        distance += self.cli_split[j - 1].dnext;
                    }
                    let cost = distance
                        + self.cli_split[j].dx_0
                        + params.penalty_capacity * (load - params.vehicle_capacity).max(0.0)
                        + params.penalty_duration
                            * (distance + self.cli_split[j].dx_0 + service_duration
                                - params.duration_limit)
                                .max(0.0);
                    if self.potential[self.idx(0, i)] + cost < self.potential[self.idx(0, j)] {
                        self.potential[self.idx(0, j)] = self.potential[self.idx(0, i)] + cost;
                        self.pred[self.idx(0, j)] = i;
                    }
                    j += 1;
                }
            }
        } else {
            let mut queue = TrivialDeque::new(params.nb_clients + 1, 0);
            for i in 1..=params.nb_clients {
                // The front is the best predecessor for i
                self.potential[self.idx(0, i)] = self.propagate(queue.get_front(), i, 0, params);
                self.pred[self.idx(0, i)] = queue.get_front();

                if i < params.nb_clients {
                    // If i is not dominated by the last of the pile
                    if !self.dominates(queue.get_back(), i, 0, params) {
                        // then i will be inserted, need to remove whoever is dominated by i
                        while queue.size() > 0 && self.dominates_right(queue.get_back(), i, 0) {
                            queue.pop_back();
                        }
                        queue.push_back(i);
                    }
                    // Check iteratively if front is dominated by the next front
                    while queue.size() > 1
                        && self.propagate(queue.get_front(), i + 1, 0, params)
                            > self.propagate(queue.get_next_front(), i + 1, 0, params) - MY_EPSILON
                    {
                        queue.pop_front();
                    }
                }
            }
        }

        if self.potential[self.idx(0, params.nb_clients)] > 1.0e29 {
            panic!("ERROR : no Split solution has been propagated until the last node");
        }

        // Filling the chromR structure
        for k in self.max_vehicles..params.nb_vehicles {
            indiv.chrom_r[k].clear();
        }

        let mut end = params.nb_clients;
        for k in (0..self.max_vehicles).rev() {
            indiv.chrom_r[k].clear();
            let begin = self.pred[self.idx(0, end)];
            for ii in begin..end {
                indiv.chrom_r[k].push(indiv.chrom_t[ii]);
            }
            end = begin;
        }

        // Return OK in case the Split algorithm reached the beginning of the routes
        end == 0
    }

    /// Split for limited fleet. Returns true if the algorithm reached the beginning of the routes.
    fn split_lf(&mut self, params: &Params, indiv: &mut Individual) -> bool {
        // Initialize the potential structure
        self.potential[self.idx(0, 0)] = 0.0;
        for k in 0..=self.max_vehicles {
            for i in 1..=params.nb_clients {
                self.potential[self.idx(k, i)] = 1.0e30;
            }
        }

        // MAIN ALGORITHM -- Simple Split using Bellman's algorithm in topological order
        if params.is_duration_constraint {
            for k in 0..self.max_vehicles {
                for i in k..params.nb_clients {
                    if self.potential[self.idx(k, i)] >= 1.0e29 {
                        continue;
                    }
                    let mut load = 0.0;
                    let mut service_duration = 0.0;
                    let mut distance = 0.0;
                    // Setting a maximum limit on load infeasibility to accelerate the algorithm
                    let mut j = i + 1;
                    while j <= params.nb_clients && load <= 1.5 * params.vehicle_capacity {
                        load += self.cli_split[j].demand;
                        service_duration += self.cli_split[j].service_time;
                        if j == i + 1 {
                            distance += self.cli_split[j].d0_x;
                        } else {
                            distance += self.cli_split[j - 1].dnext;
                        }
                        let cost = distance
                            + self.cli_split[j].dx_0
                            + params.penalty_capacity * (load - params.vehicle_capacity).max(0.0)
                            + params.penalty_duration
                                * (distance + self.cli_split[j].dx_0 + service_duration
                                    - params.duration_limit)
                                    .max(0.0);
                        if self.potential[self.idx(k, i)] + cost
                            < self.potential[self.idx(k + 1, j)]
                        {
                            self.potential[self.idx(k + 1, j)] =
                                self.potential[self.idx(k, i)] + cost;
                            self.pred[self.idx(k + 1, j)] = i;
                        }
                        j += 1;
                    }
                }
            }
        } else {
            // Without duration constraints in O(n), from "Vidal, T. (2016).
            // Split algorithm in O(n) for the capacitated vehicle routing problem. C&OR"
            let mut queue = TrivialDeque::new(params.nb_clients + 1, 0);
            for k in 0..self.max_vehicles {
                // In the Split problem there is always one feasible solution with k routes
                // that reaches the index k in the tour
                queue.reset(k);

                // The range of potentials < 1.e29 is always an interval.
                // The size of the queue will stay >= 1 until we reach the end of this interval.
                let mut i = k + 1;
                while i <= params.nb_clients && queue.size() > 0 {
                    // The front is the best predecessor for i
                    self.potential[self.idx(k + 1, i)] =
                        self.propagate(queue.get_front(), i, k, params);
                    self.pred[self.idx(k + 1, i)] = queue.get_front();

                    if i < params.nb_clients {
                        // If i is not dominated by the last of the pile
                        if !self.dominates(queue.get_back(), i, k, params) {
                            // then i will be inserted, need to remove whoever it dominates
                            while queue.size() > 0 && self.dominates_right(queue.get_back(), i, k) {
                                queue.pop_back();
                            }
                            queue.push_back(i);
                        }

                        // Check iteratively if front is dominated by the next front
                        while queue.size() > 1
                            && self.propagate(queue.get_front(), i + 1, k, params)
                                > self.propagate(queue.get_next_front(), i + 1, k, params)
                                    - MY_EPSILON
                        {
                            queue.pop_front();
                        }
                    }
                    i += 1;
                }
            }
        }

        if self.potential[self.idx(self.max_vehicles, params.nb_clients)] > 1.0e29 {
            panic!("ERROR : no Split solution has been propagated until the last node");
        }

        // It could be cheaper to use a smaller number of vehicles
        let mut min_cost = self.potential[self.idx(self.max_vehicles, params.nb_clients)];
        let mut nb_routes = self.max_vehicles;
        for k in 1..self.max_vehicles {
            if self.potential[self.idx(k, params.nb_clients)] < min_cost {
                min_cost = self.potential[self.idx(k, params.nb_clients)];
                nb_routes = k;
            }
        }

        // Filling the chromR structure
        for k in nb_routes..params.nb_vehicles {
            indiv.chrom_r[k].clear();
        }

        let mut end = params.nb_clients;
        for k in (0..nb_routes).rev() {
            indiv.chrom_r[k].clear();
            let begin = self.pred[self.idx(k + 1, end)];
            for ii in begin..end {
                indiv.chrom_r[k].push(indiv.chrom_t[ii]);
            }
            end = begin;
        }

        // Return OK in case the Split algorithm reached the beginning of the routes
        end == 0
    }
}

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

I'm catching a critical bug in my Rust translation — the C++ loop condition breaks when the potential value exceeds the threshold, but my Rust code used `continue` instead, which would skip iterations rather than stopping the loop entirely. The valid potential range ends once we hit that boundary.发现 C++ 的 `split_lf` 函数中 `potential < 1.e29` 的逻辑错误——这是循环条件而非 break 语句。现在修正这个问题。

</details>

<details>
<summary>Tool Call: rust/src/split.rs</summary>

*Status: Completed*

Diff: rust/src/split.rs
```
//! Algorithms to decode solutions represented as giant tours into complete CVRP solutions.
//!
//! Direct port of the C++ implementation, itself based on the linear Split of
//! "Vidal, T. (2016). Split algorithm in O(n) for the capacitated vehicle routing problem".

use crate::individual::Individual;
use crate::params::Params;
use crate::util::MY_EPSILON;

#[derive(Clone, Default)]
struct ClientSplit {
    demand: f64,
    service_time: f64,
    d0_x: f64,
    dx_0: f64,
    dnext: f64,
}

/// Simple deque used by the linear Split algorithms.
/// Cursors are signed so that the back can transiently move before the front.
struct TrivialDeque {
    elements: Vec<usize>,
    index_front: i32,
    index_back: i32,
}

impl TrivialDeque {
    fn new(nb_elements: usize, first_node: usize) -> Self {
        let mut elements = vec![0; nb_elements];
        elements[0] = first_node;
        Self {
            elements,
            index_front: 0,
            index_back: 0,
        }
    }

    #[inline]
    fn pop_front(&mut self) {
        self.index_front += 1;
    }

    #[inline]
    fn pop_back(&mut self) {
        self.index_back -= 1;
    }

    #[inline]
    fn push_back(&mut self, i: usize) {
        self.index_back += 1;
        self.elements[self.index_back as usize] = i;
    }

    #[inline]
    fn get_front(&self) -> usize {
        self.elements[self.index_front as usize]
    }

    #[inline]
    fn get_next_front(&self) -> usize {
        self.elements[(self.index_front + 1) as usize]
    }

    #[inline]
    fn get_back(&self) -> usize {
        self.elements[self.index_back as usize]
    }

    fn reset(&mut self, first_node: usize) {
        self.elements[0] = first_node;
        self.index_back = 0;
        self.index_front = 0;
    }

    #[inline]
    fn size(&self) -> i32 {
        self.index_back - self.index_front + 1
    }
}

pub struct Split {
    nb_clients: usize,
    nb_vehicles: usize,
    max_vehicles: usize,

    /* Auxiliary data structures to run the Linear Split algorithm */
    cli_split: Vec<ClientSplit>,
    /// Potential vector, flat (nb_vehicles + 1) x (nb_clients + 1).
    potential: Vec<f64>,
    /// Index of the predecessor in an optimal path, same layout as `potential`.
    pred: Vec<usize>,
    /// sum_distance[i] for i > 1 contains the sum of distances: sum_{k=1}^{i-1} d_{k,k+1}.
    sum_distance: Vec<f64>,
    /// sum_load[i] for i >= 1 contains the sum of loads: sum_{k=1}^{i} q_k.
    sum_load: Vec<f64>,
    /// sum_service[i] for i >= 1 contains the sum of service time: sum_{k=1}^{i} s_k.
    sum_service: Vec<f64>,
}

impl Split {
    pub fn new(params: &Params) -> Self {
        let nb_clients = params.nb_clients;
        let nb_vehicles = params.nb_vehicles;
        Self {
            nb_clients,
            nb_vehicles,
            max_vehicles: 0,
            cli_split: vec![ClientSplit::default(); nb_clients + 1],
            potential: vec![1.0e30; (nb_vehicles + 1) * (nb_clients + 1)],
            pred: vec![0; (nb_vehicles + 1) * (nb_clients + 1)],
            sum_distance: vec![0.0; nb_clients + 1],
            sum_load: vec![0.0; nb_clients + 1],
            sum_service: vec![0.0; nb_clients + 1],
        }
    }

    #[inline]
    fn idx(&self, k: usize, i: usize) -> usize {
        k * (self.nb_clients + 1) + i
    }

    /// Computes the cost of propagating the label i until j (to be called with i < j only).
    #[inline]
    fn propagate(&self, i: usize, j: usize, k: usize, params: &Params) -> f64 {
        self.potential[self.idx(k, i)] + self.sum_distance[j] - self.sum_distance[i + 1]
            + self.cli_split[i + 1].d0_x
            + self.cli_split[j].dx_0
            + params.penalty_capacity
                * (self.sum_load[j] - self.sum_load[i] - params.vehicle_capacity).max(0.0)
    }

    /// Tests if i dominates j as a predecessor for all nodes x >= j + 1 (assuming i < j).
    #[inline]
    fn dominates(&self, i: usize, j: usize, k: usize, params: &Params) -> bool {
        self.potential[self.idx(k, j)] + self.cli_split[j + 1].d0_x
            > self.potential[self.idx(k, i)] + self.cli_split[i + 1].d0_x + self.sum_distance[j + 1]
                - self.sum_distance[i + 1]
                + params.penalty_capacity * (self.sum_load[j] - self.sum_load[i])
    }

    /// Tests if j dominates i as a predecessor for all nodes x >= j + 1 (assuming i < j).
    #[inline]
    fn dominates_right(&self, i: usize, j: usize, k: usize) -> bool {
        self.potential[self.idx(k, j)] + self.cli_split[j + 1].d0_x
            < self.potential[self.idx(k, i)] + self.cli_split[i + 1].d0_x + self.sum_distance[j + 1]
                - self.sum_distance[i + 1]
                + MY_EPSILON
    }

    /// General Split function: tests the unlimited fleet Split first, and only if it
    /// does not produce a feasible solution, runs the Split algorithm for a limited fleet.
    pub fn general_split(
        &mut self,
        params: &Params,
        indiv: &mut Individual,
        nb_max_vehicles: usize,
    ) {
        // Do not apply Split with fewer vehicles than the trivial (LP) bin packing bound
        self.max_vehicles =
            nb_max_vehicles.max((params.total_demand / params.vehicle_capacity).ceil() as usize);

        // Initialization of the data structures for the linear split algorithms
        for i in 1..=params.nb_clients {
            let client = indiv.chrom_t[i - 1];
            self.cli_split[i].demand = params.clients[client].demand;
            self.cli_split[i].service_time = params.clients[client].service_duration;
            self.cli_split[i].d0_x = params.time_cost.get(0, client);
            self.cli_split[i].dx_0 = params.time_cost.get(client, 0);
            self.cli_split[i].dnext = if i < params.nb_clients {
                params.time_cost.get(client, indiv.chrom_t[i])
            } else {
                -1.0e30
            };
            self.sum_load[i] = self.sum_load[i - 1] + self.cli_split[i].demand;
            self.sum_service[i] = self.sum_service[i - 1] + self.cli_split[i].service_time;
            self.sum_distance[i] = self.sum_distance[i - 1] + self.cli_split[i - 1].dnext;
        }

        // We first try the simple split, and then the Split with limited fleet if not successful
        if !self.split_simple(params, indiv) {
            self.split_lf(params, indiv);
        }

        // Build up the rest of the Individual structure
        indiv.evaluate_complete_cost(params);
    }

    /// Split for unlimited fleet. Returns true if the algorithm reached the beginning of the routes.
    fn split_simple(&mut self, params: &Params, indiv: &mut Individual) -> bool {
        // Reinitialize the potential structure
        self.potential[self.idx(0, 0)] = 0.0;
        for i in 1..=params.nb_clients {
            self.potential[self.idx(0, i)] = 1.0e30;
        }

        // MAIN ALGORITHM -- Simple Split using Bellman's algorithm in topological order.
        // This code has been maintained as it is very simple and can be easily adapted to
        // a variety of constraints, whereas the O(n) Split has a more restricted scope.
        if params.is_duration_constraint {
            for i in 0..params.nb_clients {
                let mut load = 0.0;
                let mut distance = 0.0;
                let mut service_duration = 0.0;
                let mut j = i + 1;
                while j <= params.nb_clients && load <= 1.5 * params.vehicle_capacity {
                    load += self.cli_split[j].demand;
                    service_duration += self.cli_split[j].service_time;
                    if j == i + 1 {
                        distance += self.cli_split[j].d0_x;
                    } else {
                        distance += self.cli_split[j - 1].dnext;
                    }
                    let cost = distance
                        + self.cli_split[j].dx_0
                        + params.penalty_capacity * (load - params.vehicle_capacity).max(0.0)
                        + params.penalty_duration
                            * (distance + self.cli_split[j].dx_0 + service_duration
                                - params.duration_limit)
                                .max(0.0);
                    if self.potential[self.idx(0, i)] + cost < self.potential[self.idx(0, j)] {
                        self.potential[self.idx(0, j)] = self.potential[self.idx(0, i)] + cost;
                        self.pred[self.idx(0, j)] = i;
                    }
                    j += 1;
                }
            }
        } else {
            let mut queue = TrivialDeque::new(params.nb_clients + 1, 0);
            for i in 1..=params.nb_clients {
                // The front is the best predecessor for i
                self.potential[self.idx(0, i)] = self.propagate(queue.get_front(), i, 0, params);
                self.pred[self.idx(0, i)] = queue.get_front();

                if i < params.nb_clients {
                    // If i is not dominated by the last of the pile
                    if !self.dominates(queue.get_back(), i, 0, params) {
                        // then i will be inserted, need to remove whoever is dominated by i
                        while queue.size() > 0 && self.dominates_right(queue.get_back(), i, 0) {
                            queue.pop_back();
                        }
                        queue.push_back(i);
                    }
                    // Check iteratively if front is dominated by the next front
                    while queue.size() > 1
                        && self.propagate(queue.get_front(), i + 1, 0, params)
                            > self.propagate(queue.get_next_front(), i + 1, 0, params) - MY_EPSILON
                    {
                        queue.pop_front();
                    }
                }
            }
        }

        if self.potential[self.idx(0, params.nb_clients)] > 1.0e29 {
            panic!("ERROR : no Split solution has been propagated until the last node");
        }

        // Filling the chromR structure
        for k in self.max_vehicles..params.nb_vehicles {
            indiv.chrom_r[k].clear();
        }

        let mut end = params.nb_clients;
        for k in (0..self.max_vehicles).rev() {
            indiv.chrom_r[k].clear();
            let begin = self.pred[self.idx(0, end)];
            for ii in begin..end {
                indiv.chrom_r[k].push(indiv.chrom_t[ii]);
            }
            end = begin;
        }

        // Return OK in case the Split algorithm reached the beginning of the routes
        end == 0
    }

    /// Split for limited fleet. Returns true if the algorithm reached the beginning of the routes.
    fn split_lf(&mut self, params: &Params, indiv: &mut Individual) -> bool {
        // Initialize the potential structure
        self.potential[self.idx(0, 0)] = 0.0;
        for k in 0..=self.max_vehicles {
            for i in 1..=params.nb_clients {
                self.potential[self.idx(k, i)] = 1.0e30;
            }
        }

        // MAIN ALGORITHM -- Simple Split using Bellman's algorithm in topological order
        if params.is_duration_constraint {
            for k in 0..self.max_vehicles {
                for i in k..params.nb_clients {
                    // The loop stops as soon as the potential is unreachable (interval property)
                    if self.potential[self.idx(k, i)] >= 1.0e29 {
                        break;
                    }
                    let mut load = 0.0;
                    let mut service_duration = 0.0;
                    let mut distance = 0.0;
                    // Setting a maximum limit on load infeasibility to accelerate the algorithm
                    let mut j = i + 1;
                    while j <= params.nb_clients && load <= 1.5 * params.vehicle_capacity {
                        load += self.cli_split[j].demand;
                        service_duration += self.cli_split[j].service_time;
                        if j == i + 1 {
                            distance += self.cli_split[j].d0_x;
                        } else {
                            distance += self.cli_split[j - 1].dnext;
                        }
                        let cost = distance
                            + self.cli_split[j].dx_0
                            + params.penalty_capacity * (load - params.vehicle_capacity).max(0.0)
                            + params.penalty_duration
                                * (distance + self.cli_split[j].dx_0 + service_duration
                                    - params.duration_limit)
                                    .max(0.0);
                        if self.potential[self.idx(k, i)] + cost
                            < self.potential[self.idx(k + 1, j)]
                        {
                            self.potential[self.idx(k + 1, j)] =
                                self.potential[self.idx(k, i)] + cost;
                            self.pred[self.idx(k + 1, j)] = i;
                        }
                        j += 1;
                    }
                }
            }
        } else {
            // Without duration constraints in O(n), from "Vidal, T. (2016).
            // Split algorithm in O(n) for the capacitated vehicle routing problem. C&OR"
            let mut queue = TrivialDeque::new(params.nb_clients + 1, 0);
            for k in 0..self.max_vehicles {
                // In the Split problem there is always one feasible solution with k routes
                // that reaches the index k in the tour
                queue.reset(k);

                // The range of potentials < 1.e29 is always an interval.
                // The size of the queue will stay >= 1 until we reach the end of this interval.
                let mut i = k + 1;
                while i <= params.nb_clients && queue.size() > 0 {
                    // The front is the best predecessor for i
                    self.potential[self.idx(k + 1, i)] =
                        self.propagate(queue.get_front(), i, k, params);
                    self.pred[self.idx(k + 1, i)] = queue.get_front();

                    if i < params.nb_clients {
                        // If i is not dominated by the last of the pile
                        if !self.dominates(queue.get_back(), i, k, params) {
                            // then i will be inserted, need to remove whoever it dominates
                            while queue.size() > 0 && self.dominates_right(queue.get_back(), i, k) {
                                queue.pop_back();
                            }
                            queue.push_back(i);
                        }

                        // Check iteratively if front is dominated by the next front
                        while queue.size() > 1
                            && self.propagate(queue.get_front(), i + 1, k, params)
                                > self.propagate(queue.get_next_front(), i + 1, k, params)
                                    - MY_EPSILON
                        {
                            queue.pop_front();
                        }
                    }
                    i += 1;
                }
            }
        }

        if self.potential[self.idx(self.max_vehicles, params.nb_clients)] > 1.0e29 {
            panic!("ERROR : no Split solution has been propagated until the last node");
        }

        // It could be cheaper to use a smaller number of vehicles
        let mut min_cost = self.potential[self.idx(self.max_vehicles, params.nb_clients)];
        let mut nb_routes = self.max_vehicles;
        for k in 1..self.max_vehicles {
            if self.potential[self.idx(k, params.nb_clients)] < min_cost {
                min_cost = self.potential[self.idx(k, params.nb_clients)];
                nb_routes = k;
            }
        }

        // Filling the chromR structure
        for k in nb_routes..params.nb_vehicles {
            indiv.chrom_r[k].clear();
        }

        let mut end = params.nb_clients;
        for k in (0..nb_routes).rev() {
            indiv.chrom_r[k].clear();
            let begin = self.pred[self.idx(k + 1, end)];
            for ii in begin..end {
                indiv.chrom_r[k].push(indiv.chrom_t[ii]);
            }
            end = begin;
        }

        // Return OK in case the Split algorithm reached the beginning of the routes
        end == 0
    }
}

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

Right, so the C++ loop condition checks if the potential value exceeds the threshold and breaks accordingly. Now I'm moving on to rewrite the local_search.rs file carefully.

</details>

现在写最核心的 `local_search.rs`(链表用索引 arena 实现,完整移植 9 个移动算子 + SWAP*):

<details>
<summary>Tool Call: rust/src/local\_search.rs</summary>

*Status: Completed*

Diff: rust/src/local_search.rs
```
//! Local search functions, including the SWAP* neighborhood.
//!
//! The C++ implementation represents the solution as a doubly linked list of `Node*`.
//! Here the nodes live in a single arena (`Vec<Node>`) and links are arena indices,
//! which keeps the same O(1) updates without any reference counting or unsafe code.
//!
//! Arena layout: indices `0..=nb_clients` are the client nodes (0 is a sentinel),
//! then one start depot per route, then one end depot per route.

use std::collections::BTreeSet;

use crate::circle_sector::CircleSector;
use crate::individual::Individual;
use crate::params::Params;
use crate::util::MY_EPSILON;

#[derive(Clone, Default)]
struct Node {
    /// Tells whether this node represents a depot or not.
    is_depot: bool,
    /// Node index (client number, 0 for depots): used for distance matrix lookups.
    cour: usize,
    /// Position in the route.
    position: usize,
    /// "When" the RI moves for this node have been last tested.
    when_last_tested_ri: i32,
    /// Next node in the route order (arena index).
    next: usize,
    /// Previous node in the route order (arena index).
    prev: usize,
    /// Associated route index.
    route: usize,
    /// Cumulated load on this route until the customer (including itself).
    cumulated_load: f64,
    /// Cumulated time on this route until the customer (including itself).
    cumulated_time: f64,
    /// Difference of cost if the segment (0...cour) is reversed (useful for 2-opt asymmetric).
    cumulated_reversal_distance: f64,
    /// Difference of cost in the current route if the node is removed (used in SWAP*).
    delta_removal: f64,
}

#[derive(Clone, Default)]
struct Route {
    /// Route index.
    cour: usize,
    /// Number of customers visited in the route.
    nb_customers: usize,
    /// "When" this route has been last modified.
    when_last_modified: i32,
    /// "When" the SWAP* moves for this route have been last tested.
    when_last_tested_swap_star: i32,
    /// Arena index of the associated start depot.
    depot: usize,
    /// Total time on the route.
    duration: f64,
    /// Total load on the route.
    load: f64,
    /// Difference of cost if the route is reversed.
    reversal_distance: f64,
    /// Current sum of load and duration penalties.
    penalty: f64,
    /// Polar angle of the barycenter of the route.
    polar_angle_barycenter: f64,
    /// Circle sector associated to the set of customers.
    sector: CircleSector,
}

/// Structure used in SWAP* to remember the three best insertion positions
/// of a customer in a given route.
#[derive(Clone)]
struct ThreeBestInsert {
    when_last_calculated: i32,
    best_cost: [f64; 3],
    best_location: [Option<usize>; 3],
}

impl Default for ThreeBestInsert {
    fn default() -> Self {
        Self {
            when_last_calculated: 0,
            best_cost: [1.0e30; 3],
            best_location: [None; 3],
        }
    }
}

impl ThreeBestInsert {
    fn compare_and_add(&mut self, cost_insert: f64, place_insert: usize) {
        if cost_insert >= self.best_cost[2] {
            return;
        }
        if cost_insert >= self.best_cost[1] {
            self.best_cost[2] = cost_insert;
            self.best_location[2] = Some(place_insert);
        } else if cost_insert >= self.best_cost[0] {
            self.best_cost[2] = self.best_cost[1];
            self.best_location[2] = self.best_location[1];
            self.best_cost[1] = cost_insert;
            self.best_location[1] = Some(place_insert);
        } else {
            self.best_cost[2] = self.best_cost[1];
            self.best_location[2] = self.best_location[1];
            self.best_cost[1] = self.best_cost[0];
            self.best_location[1] = self.best_location[0];
            self.best_cost[0] = cost_insert;
            self.best_location[0] = Some(place_insert);
        }
    }

    /// Resets the structure (no insertion calculated).
    fn reset(&mut self) {
        self.best_cost = [1.0e30; 3];
        self.best_location = [None; 3];
    }
}

/// Structure used to keep track of the best SWAP* move.
struct SwapStarElement {
    move_cost: f64,
    u: Option<usize>,
    best_position_u: Option<usize>,
    v: Option<usize>,
    best_position_v: Option<usize>,
}

impl Default for SwapStarElement {
    fn default() -> Self {
        Self {
            move_cost: 1.0e30,
            u: None,
            best_position_u: None,
            v: None,
            best_position_v: None,
        }
    }
}

/// Main local search structure.
pub struct LocalSearch {
    nb_clients: usize,
    nb_vehicles: usize,

    /// Tells whether all moves have been evaluated without success.
    search_completed: bool,
    /// Total number of moves (RI and SWAP*) applied during the local search.
    /// This is not only a counter, it is also used to avoid repeating move evaluations.
    nb_moves: i32,
    /// Randomized order for checking the nodes in the RI local search.
    order_nodes: Vec<usize>,
    /// Randomized order for checking the routes in the SWAP* local search.
    order_routes: Vec<usize>,
    /// Indices of all empty routes.
    empty_routes: BTreeSet<usize>,
    /// Current loop index.
    loop_id: i32,

    /* THE SOLUTION IS REPRESENTED AS A LINKED LIST OF ELEMENTS */
    nodes: Vec<Node>,
    routes: Vec<Route>,
    /// (SWAP*) For each route and node, the cheapest insertion cost, flat layout
    /// `route * (nb_clients + 1) + client`.
    best_insert_client: Vec<ThreeBestInsert>,

    /* TEMPORARY VARIABLES USED IN THE LOCAL SEARCH LOOPS */
    // node_u_prev -> node_u -> node_x -> node_x_next
    // node_v_prev -> node_v -> node_y -> node_y_next
    node_u: usize,
    node_x: usize,
    node_v: usize,
    node_y: usize,
    route_u: usize,
    route_v: usize,
    node_u_prev_index: usize,
    node_u_index: usize,
    node_x_index: usize,
    node_x_next_index: usize,
    node_v_prev_index: usize,
    node_v_index: usize,
    node_y_index: usize,
    node_y_next_index: usize,
    load_u: f64,
    load_x: f64,
    load_v: f64,
    load_y: f64,
    service_u: f64,
    service_x: f64,
    service_v: f64,
    service_y: f64,
    penalty_capacity_ls: f64,
    penalty_duration_ls: f64,
    intra_route_move: bool,
}

impl LocalSearch {
    pub fn new(params: &Params) -> Self {
        let nb_clients = params.nb_clients;
        let nb_vehicles = params.nb_vehicles;

        let mut nodes = vec![Node::default(); nb_clients + 1 + 2 * nb_vehicles];
        for (i, node) in nodes.iter_mut().enumerate().take(nb_clients + 1) {
            node.cour = i;
            node.is_depot = false;
        }
        let mut routes = Vec::with_capacity(nb_vehicles);
        for r in 0..nb_vehicles {
            let depot = nb_clients + 1 + r;
            let depot_end = nb_clients + 1 + nb_vehicles + r;
            nodes[depot].cour = 0;
            nodes[depot].is_depot = true;
            nodes[depot].route = r;
            nodes[depot_end].cour = 0;
            nodes[depot_end].is_depot = true;
            nodes[depot_end].route = r;
            routes.push(Route {
                cour: r,
                depot,
                ..Route::default()
            });
        }

        Self {
            nb_clients,
            nb_vehicles,
            search_completed: false,
            nb_moves: 0,
            order_nodes: (1..=nb_clients).collect(),
            order_routes: (0..nb_vehicles).collect(),
            empty_routes: BTreeSet::new(),
            loop_id: 0,
            nodes,
            routes,
            best_insert_client: vec![ThreeBestInsert::default(); nb_vehicles * (nb_clients + 1)],
            node_u: 0,
            node_x: 0,
            node_v: 0,
            node_y: 0,
            route_u: 0,
            route_v: 0,
            node_u_prev_index: 0,
            node_u_index: 0,
            node_x_index: 0,
            node_x_next_index: 0,
            node_v_prev_index: 0,
            node_v_index: 0,
            node_y_index: 0,
            node_y_next_index: 0,
            load_u: 0.0,
            load_x: 0.0,
            load_v: 0.0,
            load_y: 0.0,
            service_u: 0.0,
            service_x: 0.0,
            service_v: 0.0,
            service_y: 0.0,
            penalty_capacity_ls: 0.0,
            penalty_duration_ls: 0.0,
            intra_route_move: false,
        }
    }

    #[inline]
    fn depot_node(&self, route: usize) -> usize {
        self.nb_clients + 1 + route
    }

    #[inline]
    fn depot_end_node(&self, route: usize) -> usize {
        self.nb_clients + 1 + self.nb_vehicles + route
    }

    #[inline]
    fn bi_index(&self, route: usize, client: usize) -> usize {
        route * (self.nb_clients + 1) + client
    }

    #[inline]
    fn penalty_excess_duration(&self, params: &Params, duration: f64) -> f64 {
        (duration - params.duration_limit).max(0.0) * self.penalty_duration_ls
    }

    #[inline]
    fn penalty_excess_load(&self, params: &Params, load: f64) -> f64 {
        (load - params.vehicle_capacity).max(0.0) * self.penalty_capacity_ls
    }

    /// Runs the local search with the specified penalty values.
    pub fn run(
        &mut self,
        params: &mut Params,
        indiv: &mut Individual,
        penalty_capacity_ls: f64,
        penalty_duration_ls: f64,
    ) {
        self.penalty_capacity_ls = penalty_capacity_ls;
        self.penalty_duration_ls = penalty_duration_ls;
        self.load_individual(params, indiv);

        // Shuffling the order of the nodes explored by the LS to allow for more diversity in the search
        params.rng.shuffle(&mut self.order_nodes);
        params.rng.shuffle(&mut self.order_routes);
        {
            let nb_granular = params.ap.nb_granular as u32;
            let Params {
                rng,
                correlated_vertices,
                ..
            } = params;
            for i in 1..=self.nb_clients {
                // O(n/nbGranular) shuffles on average, to keep linear-time complexity overall
                if rng.next() % nb_granular == 0 {
                    rng.shuffle(&mut correlated_vertices[i]);
                }
            }
        }
        let params: &Params = params;

        self.search_completed = false;
        self.loop_id = 0;
        while !self.search_completed {
            // Allows at least two loops since some moves involving empty routes are not checked at the first loop
            if self.loop_id > 1 {
                self.search_completed = true;
            }

            /* CLASSICAL ROUTE IMPROVEMENT (RI) MOVES SUBJECT TO A PROXIMITY RESTRICTION */
            for pos_u in 0..self.nb_clients {
                self.node_u = self.order_nodes[pos_u];
                let last_test_ri_node_u = self.nodes[self.node_u].when_last_tested_ri;
                self.nodes[self.node_u].when_last_tested_ri = self.nb_moves;
                let u_client = self.nodes[self.node_u].cour;
                'pos_v: for pos_v in 0..params.correlated_vertices[u_client].len() {
                    self.node_v = params.correlated_vertices[u_client][pos_v];
                    let route_u_modified =
                        self.routes[self.nodes[self.node_u].route].when_last_modified;
                    let route_v_modified =
                        self.routes[self.nodes[self.node_v].route].when_last_modified;
                    // Only evaluate moves involving routes that have been modified since
                    // the last move evaluations for node_u
                    if self.loop_id == 0
                        || route_u_modified.max(route_v_modified) > last_test_ri_node_u
                    {
                        // Randomizing the order of the neighborhoods within this loop does not matter much
                        // as we are already randomizing the order of the node pairs (and it's not very common
                        // to find improving moves of different types for the same node pair)
                        self.set_local_variables_route_u(params);
                        self.set_local_variables_route_v(params);
                        if self.move1(params) {
                            continue 'pos_v;
                        } // RELOCATE
                        if self.move2(params) {
                            continue 'pos_v;
                        } // RELOCATE
                        if self.move3(params) {
                            continue 'pos_v;
                        } // RELOCATE
                        if self.node_u_index <= self.node_v_index && self.move4(params) {
                            continue 'pos_v;
                        } // SWAP
                        if self.move5(params) {
                            continue 'pos_v;
                        } // SWAP
                        if self.node_u_index <= self.node_v_index && self.move6(params) {
                            continue 'pos_v;
                        } // SWAP
                        if self.intra_route_move && self.move7(params) {
                            continue 'pos_v;
                        } // 2-OPT
                        if !self.intra_route_move && self.move8(params) {
                            continue 'pos_v;
                        } // 2-OPT*
                        if !self.intra_route_move && self.move9(params) {
                            continue 'pos_v;
                        } // 2-OPT*

                        // Trying moves that insert node_u directly after the depot
                        if self.nodes[self.nodes[self.node_v].prev].is_depot {
                            self.node_v = self.nodes[self.node_v].prev;
                            self.set_local_variables_route_v(params);
                            if self.move1(params) {
                                continue 'pos_v;
                            } // RELOCATE
                            if self.move2(params) {
                                continue 'pos_v;
                            } // RELOCATE
                            if self.move3(params) {
                                continue 'pos_v;
                            } // RELOCATE
                            if !self.intra_route_move && self.move8(params) {
                                continue 'pos_v;
                            } // 2-OPT*
                            if !self.intra_route_move && self.move9(params) {
                                continue 'pos_v;
                            } // 2-OPT*
                        }
                    }
                }

                /* MOVES INVOLVING AN EMPTY ROUTE -- NOT TESTED IN THE FIRST LOOP TO AVOID INCREASING TOO MUCH THE FLEET SIZE */
                if self.loop_id > 0 && !self.empty_routes.is_empty() {
                    let empty_route = *self.empty_routes.iter().next().unwrap();
                    self.node_v = self.routes[empty_route].depot;
                    self.set_local_variables_route_u(params);
                    self.set_local_variables_route_v(params);
                    if self.move1(params) {
                        continue;
                    } // RELOCATE
                    if self.move2(params) {
                        continue;
                    } // RELOCATE
                    if self.move3(params) {
                        continue;
                    } // RELOCATE
                    if self.move9(params) {
                        continue;
                    } // 2-OPT*
                }
            }

            if params.ap.use_swap_star && params.are_coordinates_provided {
                /* (SWAP*) MOVES LIMITED TO ROUTE PAIRS WHOSE CIRCLE SECTORS OVERLAP */
                for r_u in 0..self.nb_vehicles {
                    self.route_u = self.order_routes[r_u];
                    let last_test_swap_star_route_u =
                        self.routes[self.route_u].when_last_tested_swap_star;
                    self.routes[self.route_u].when_last_tested_swap_star = self.nb_moves;
                    for r_v in 0..self.nb_vehicles {
                        self.route_v = self.order_routes[r_v];
                        let route_u = &self.routes[self.route_u];
                        let route_v = &self.routes[self.route_v];
                        if route_u.nb_customers > 0
                            && route_v.nb_customers > 0
                            && route_u.cour < route_v.cour
                            && (self.loop_id == 0
                                || route_u.when_last_modified.max(route_v.when_last_modified)
                                    > last_test_swap_star_route_u)
                            && CircleSector::overlap(&route_u.sector, &route_v.sector)
                        {
                            self.swap_star(params);
                        }
                    }
                }
            }

            self.loop_id += 1;
        }

        // Register the solution produced by the LS in the individual
        self.export_individual(params, indiv);
    }

    fn set_local_variables_route_u(&mut self, params: &Params) {
        self.route_u = self.nodes[self.node_u].route;
        self.node_x = self.nodes[self.node_u].next;
        self.node_x_next_index = self.nodes[self.nodes[self.node_x].next].cour;
        self.node_u_index = self.nodes[self.node_u].cour;
        self.node_u_prev_index = self.nodes[self.nodes[self.node_u].prev].cour;
        self.node_x_index = self.nodes[self.node_x].cour;
        self.load_u = params.clients[self.node_u_index].demand;
        self.service_u = params.clients[self.node_u_index].service_duration;
        self.load_x = params.clients[self.node_x_index].demand;
        self.service_x = params.clients[self.node_x_index].service_duration;
    }

    fn set_local_variables_route_v(&mut self, params: &Params) {
        self.route_v = self.nodes[self.node_v].route;
        self.node_y = self.nodes[self.node_v].next;
        self.node_y_next_index = self.nodes[self.nodes[self.node_y].next].cour;
        self.node_v_index = self.nodes[self.node_v].cour;
        self.node_v_prev_index = self.nodes[self.nodes[self.node_v].prev].cour;
        self.node_y_index = self.nodes[self.node_y].cour;
        self.load_v = params.clients[self.node_v_index].demand;
        self.service_v = params.clients[self.node_v_index].service_duration;
        self.load_y = params.clients[self.node_y_index].demand;
        self.service_y = params.clients[self.node_y_index].service_duration;
        self.intra_route_move = self.route_u == self.route_v;
    }

    /// If U is a client node, remove U and insert it after V.
    fn move1(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_x_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_u_index, self.node_x_index);
        let mut cost_supp_v = d(self.node_v_index, self.node_u_index)
            + d(self.node_u_index, self.node_y_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            // Early move pruning to save CPU time: this move cannot improve
            // without checking additional (load, duration...) constraints
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self
                .penalty_excess_duration(p, route_u.duration + cost_supp_u - self.service_u)
                + self.penalty_excess_load(p, route_u.load - self.load_u)
                - route_u.penalty;

            cost_supp_v += self
                .penalty_excess_duration(p, route_v.duration + cost_supp_v + self.service_u)
                + self.penalty_excess_load(p, route_v.load + self.load_u)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u_index == self.node_y_index {
            return false;
        }

        self.insert_node(self.node_u, self.node_v);
        self.nb_moves += 1; // Increment move counter before updating route data
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If U and X are client nodes, remove them and insert (U,X) after V.
    fn move2(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_x_next_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_x_index, self.node_x_next_index);
        let mut cost_supp_v = d(self.node_v_index, self.node_u_index)
            + d(self.node_x_index, self.node_y_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u
                    - d(self.node_u_index, self.node_x_index)
                    - self.service_u
                    - self.service_x,
            ) + self
                .penalty_excess_load(p, route_u.load - self.load_u - self.load_x)
                - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration
                    + cost_supp_v
                    + d(self.node_u_index, self.node_x_index)
                    + self.service_u
                    + self.service_x,
            ) + self
                .penalty_excess_load(p, route_v.load + self.load_u + self.load_x)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u == self.node_y
            || self.node_v == self.node_x
            || self.nodes[self.node_x].is_depot
        {
            return false;
        }

        self.insert_node(self.node_u, self.node_v);
        self.insert_node(self.node_x, self.node_u);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If U and X are client nodes, remove them and insert (X,U) after V.
    fn move3(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_x_next_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_u_index, self.node_x_index)
            - d(self.node_x_index, self.node_x_next_index);
        let mut cost_supp_v = d(self.node_v_index, self.node_x_index)
            + d(self.node_x_index, self.node_u_index)
            + d(self.node_u_index, self.node_y_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u - self.service_u - self.service_x,
            ) + self
                .penalty_excess_load(p, route_u.load - self.load_u - self.load_x)
                - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration + cost_supp_v + self.service_u + self.service_x,
            ) + self
                .penalty_excess_load(p, route_v.load + self.load_u + self.load_x)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u == self.node_y
            || self.node_x == self.node_v
            || self.nodes[self.node_x].is_depot
        {
            return false;
        }

        self.insert_node(self.node_x, self.node_v);
        self.insert_node(self.node_u, self.node_x);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If U and V are client nodes, swap U and V.
    fn move4(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_v_index)
            + d(self.node_v_index, self.node_x_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_u_index, self.node_x_index);
        let mut cost_supp_v = d(self.node_v_prev_index, self.node_u_index)
            + d(self.node_u_index, self.node_y_index)
            - d(self.node_v_prev_index, self.node_v_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u + self.service_v - self.service_u,
            ) + self
                .penalty_excess_load(p, route_u.load + self.load_v - self.load_u)
                - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration + cost_supp_v - self.service_v + self.service_u,
            ) + self
                .penalty_excess_load(p, route_v.load + self.load_u - self.load_v)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u_index == self.node_v_prev_index || self.node_u_index == self.node_y_index {
            return false;
        }

        self.swap_node(self.node_u, self.node_v);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If U, X and V are client nodes, swap (U,X) and V.
    fn move5(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_v_index)
            + d(self.node_v_index, self.node_x_next_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_x_index, self.node_x_next_index);
        let mut cost_supp_v = d(self.node_v_prev_index, self.node_u_index)
            + d(self.node_x_index, self.node_y_index)
            - d(self.node_v_prev_index, self.node_v_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u - d(self.node_u_index, self.node_x_index)
                    + self.service_v
                    - self.service_u
                    - self.service_x,
            ) + self
                .penalty_excess_load(p, route_u.load + self.load_v - self.load_u - self.load_x)
                - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration + cost_supp_v + d(self.node_u_index, self.node_x_index)
                    - self.service_v
                    + self.service_u
                    + self.service_x,
            ) + self
                .penalty_excess_load(p, route_v.load + self.load_u + self.load_x - self.load_v)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u == self.nodes[self.node_v].prev
            || self.node_x == self.nodes[self.node_v].prev
            || self.node_u == self.node_y
            || self.nodes[self.node_x].is_depot
        {
            return false;
        }

        self.swap_node(self.node_u, self.node_v);
        self.insert_node(self.node_x, self.node_u);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If (U,X) and (V,Y) are client nodes, swap (U,X) and (V,Y).
    fn move6(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_v_index)
            + d(self.node_y_index, self.node_x_next_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_x_index, self.node_x_next_index);
        let mut cost_supp_v = d(self.node_v_prev_index, self.node_u_index)
            + d(self.node_x_index, self.node_y_next_index)
            - d(self.node_v_prev_index, self.node_v_index)
            - d(self.node_y_index, self.node_y_next_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u - d(self.node_u_index, self.node_x_index)
                    + d(self.node_v_index, self.node_y_index)
                    + self.service_v
                    + self.service_y
                    - self.service_u
                    - self.service_x,
            ) + self.penalty_excess_load(
                p,
                route_u.load + self.load_v + self.load_y - self.load_u - self.load_x,
            ) - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration + cost_supp_v + d(self.node_u_index, self.node_x_index)
                    - d(self.node_v_index, self.node_y_index)
                    - self.service_v
                    - self.service_y
                    + self.service_u
                    + self.service_x,
            ) + self.penalty_excess_load(
                p,
                route_v.load + self.load_u + self.load_x - self.load_v - self.load_y,
            ) - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.nodes[self.node_x].is_depot
            || self.nodes[self.node_y].is_depot
            || self.node_y == self.nodes[self.node_u].prev
            || self.node_u == self.node_y
            || self.node_x == self.node_v
            || self.node_v == self.nodes[self.node_x].next
        {
            return false;
        }

        self.swap_node(self.node_u, self.node_v);
        self.swap_node(self.node_x, self.node_y);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If route(U) == route(V), replace (U,X) and (V,Y) by (U,V) and (X,Y).
    fn move7(&mut self, p: &Params) -> bool {
        if self.nodes[self.node_u].position > self.nodes[self.node_v].position {
            return false;
        }

        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let cost = d(self.node_u_index, self.node_v_index)
            + d(self.node_x_index, self.node_y_index)
            - d(self.node_u_index, self.node_x_index)
            - d(self.node_v_index, self.node_y_index)
            + self.nodes[self.node_v].cumulated_reversal_distance
            - self.nodes[self.node_x].cumulated_reversal_distance;

        if cost > -MY_EPSILON {
            return false;
        }
        if self.nodes[self.node_u].next == self.node_v {
            return false;
        }

        // Reverse the segment between X and V
        let mut node_num = self.nodes[self.node_x].next;
        self.nodes[self.node_x].prev = node_num;
        self.nodes[self.node_x].next = self.node_y;

        while node_num != self.node_v {
            let temp = self.nodes[node_num].next;
            self.nodes[node_num].next = self.nodes[node_num].prev;
            self.nodes[node_num].prev = temp;
            node_num = temp;
        }

        self.nodes[self.node_v].next = self.nodes[self.node_v].prev;
        self.nodes[self.node_v].prev = self.node_u;
        self.nodes[self.node_u].next = self.node_v;
        self.nodes[self.node_y].prev = self.node_x;

        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        true
    }

    /// If route(U) != route(V), replace (U,X) and (V,Y) by (U,V) and (X,Y).
    fn move8(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost = {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            d(self.node_u_index, self.node_v_index) + d(self.node_x_index, self.node_y_index)
                - d(self.node_u_index, self.node_x_index)
                - d(self.node_v_index, self.node_y_index)
                + self.nodes[self.node_v].cumulated_reversal_distance
                + route_u.reversal_distance
                - self.nodes[self.node_x].cumulated_reversal_distance
                - route_u.penalty
                - route_v.penalty
        };

        // Early move pruning to save CPU time: this move cannot improve
        // without checking additional (load, duration...) constraints
        if cost >= 0.0 {
            return false;
        }

        {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            let node_u = &self.nodes[self.node_u];
            let node_v = &self.nodes[self.node_v];
            let node_x = &self.nodes[self.node_x];
            cost += self.penalty_excess_duration(
                p,
                node_u.cumulated_time
                    + node_v.cumulated_time
                    + node_v.cumulated_reversal_distance
                    + d(self.node_u_index, self.node_v_index),
            ) + self.penalty_excess_duration(
                p,
                route_u.duration - node_u.cumulated_time - d(self.node_u_index, self.node_x_index)
                    + route_u.reversal_distance
                    - node_x.cumulated_reversal_distance
                    + route_v.duration
                    - node_v.cumulated_time
                    - d(self.node_v_index, self.node_y_index)
                    + d(self.node_x_index, self.node_y_index),
            ) + self.penalty_excess_load(p, node_u.cumulated_load + node_v.cumulated_load)
                + self.penalty_excess_load(
                    p,
                    route_u.load + route_v.load - node_u.cumulated_load - node_v.cumulated_load,
                );
        }

        if cost > -MY_EPSILON {
            return false;
        }

        let depot_u = self.routes[self.route_u].depot;
        let depot_v = self.routes[self.route_v].depot;
        let depot_u_fin = self.nodes[depot_u].prev;
        let depot_v_fin = self.nodes[depot_v].prev;
        let depot_v_suiv = self.nodes[depot_v].next;

        // Reverse the tail of route U and append it to route V (and vice versa)
        let mut xx = self.node_x;
        while !self.nodes[xx].is_depot {
            let temp = self.nodes[xx].next;
            self.nodes[xx].next = self.nodes[xx].prev;
            self.nodes[xx].prev = temp;
            self.nodes[xx].route = self.route_v;
            xx = temp;
        }

        let mut vv = self.node_v;
        while !self.nodes[vv].is_depot {
            let temp = self.nodes[vv].prev;
            self.nodes[vv].prev = self.nodes[vv].next;
            self.nodes[vv].next = temp;
            self.nodes[vv].route = self.route_u;
            vv = temp;
        }

        self.nodes[self.node_u].next = self.node_v;
        self.nodes[self.node_v].prev = self.node_u;
        self.nodes[self.node_x].next = self.node_y;
        self.nodes[self.node_y].prev = self.node_x;

        if self.nodes[self.node_x].is_depot {
            self.nodes[depot_u_fin].next = depot_u;
            self.nodes[depot_u_fin].prev = depot_v_suiv;
            self.nodes[depot_v_suiv].next = depot_u_fin;
            self.nodes[depot_v].next = self.node_y;
            self.nodes[self.node_y].prev = depot_v;
        } else if self.nodes[self.node_v].is_depot {
            self.nodes[depot_v].next = self.nodes[depot_u_fin].prev;
            let new_next = self.nodes[depot_v].next;
            self.nodes[new_next].prev = depot_v;
            self.nodes[depot_v].prev = depot_v_fin;
            self.nodes[depot_u_fin].prev = self.node_u;
            self.nodes[self.node_u].next = depot_u_fin;
        } else {
            self.nodes[depot_v].next = self.nodes[depot_u_fin].prev;
            let new_next = self.nodes[depot_v].next;
            self.nodes[new_next].prev = depot_v;
            self.nodes[depot_u_fin].prev = depot_v_suiv;
            self.nodes[depot_v_suiv].next = depot_u_fin;
        }

        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        self.update_route_data(p, self.route_v);
        true
    }

    /// If route(U) != route(V), replace (U,X) and (V,Y) by (U,Y) and (V,X).
    fn move9(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost = {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            d(self.node_u_index, self.node_y_index) + d(self.node_v_index, self.node_x_index)
                - d(self.node_u_index, self.node_x_index)
                - d(self.node_v_index, self.node_y_index)
                - route_u.penalty
                - route_v.penalty
        };

        // Early move pruning to save CPU time
        if cost >= 0.0 {
            return false;
        }

        {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            let node_u = &self.nodes[self.node_u];
            let node_v = &self.nodes[self.node_v];
            cost += self.penalty_excess_duration(
                p,
                node_u.cumulated_time + route_v.duration
                    - node_v.cumulated_time
                    - d(self.node_v_index, self.node_y_index)
                    + d(self.node_u_index, self.node_y_index),
            ) + self.penalty_excess_duration(
                p,
                route_u.duration - node_u.cumulated_time - d(self.node_u_index, self.node_x_index)
                    + node_v.cumulated_time
                    + d(self.node_v_index, self.node_x_index),
            ) + self.penalty_excess_load(
                p,
                node_u.cumulated_load + route_v.load - node_v.cumulated_load,
            ) + self.penalty_excess_load(
                p,
                node_v.cumulated_load + route_u.load - node_u.cumulated_load,
            );
        }

        if cost > -MY_EPSILON {
            return false;
        }

        let depot_u = self.routes[self.route_u].depot;
        let depot_v = self.routes[self.route_v].depot;
        let depot_u_fin = self.nodes[depot_u].prev;
        let depot_v_fin = self.nodes[depot_v].prev;
        let depot_u_pred = self.nodes[depot_u_fin].prev;

        // Swap the tails of the two routes
        let mut count = self.node_y;
        while !self.nodes[count].is_depot {
            self.nodes[count].route = self.route_u;
            count = self.nodes[count].next;
        }

        count = self.node_x;
        while !self.nodes[count].is_depot {
            self.nodes[count].route = self.route_v;
            count = self.nodes[count].next;
        }

        self.nodes[self.node_u].next = self.node_y;
        self.nodes[self.node_y].prev = self.node_u;
        self.nodes[self.node_v].next = self.node_x;
        self.nodes[self.node_x].prev = self.node_v;

        if self.nodes[self.node_x].is_depot {
            self.nodes[depot_u_fin].prev = self.nodes[depot_v_fin].prev;
            let new_prev = self.nodes[depot_u_fin].prev;
            self.nodes[new_prev].next = depot_u_fin;
            self.nodes[self.node_v].next = depot_v_fin;
            self.nodes[depot_v_fin].prev = self.node_v;
        } else {
            self.nodes[depot_u_fin].prev = self.nodes[depot_v_fin].prev;
            let new_prev = self.nodes[depot_u_fin].prev;
            self.nodes[new_prev].next = depot_u_fin;
            self.nodes[depot_v_fin].prev = depot_u_pred;
            self.nodes[depot_u_pred].next = depot_v_fin;
        }

        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        self.update_route_data(p, self.route_v);
        true
    }

    /// Calculates all SWAP* moves between route_u and route_v and applies the best improving one.
    fn swap_star(&mut self, p: &Params) -> bool {
        let mut best = SwapStarElement::default();

        // Preprocessing insertion costs
        self.preprocess_insertions(p, self.route_u, self.route_v);
        self.preprocess_insertions(p, self.route_v, self.route_u);

        let route_u = self.route_u;
        let route_v = self.route_v;
        let depot_u_next = self.nodes[self.routes[route_u].depot].next;
        let depot_v_next = self.nodes[self.routes[route_v].depot].next;
        let d = |i: usize, j: usize| p.time_cost.get(i, j);

        // Evaluating the moves
        let mut u = depot_u_next;
        while !self.nodes[u].is_depot {
            let u_cour = self.nodes[u].cour;
            let mut v = depot_v_next;
            while !self.nodes[v].is_depot {
                let v_cour = self.nodes[v].cour;
                let delta_pen_route_u = self.penalty_excess_load(
                    p,
                    self.routes[route_u].load + p.clients[v_cour].demand - p.clients[u_cour].demand,
                ) - self.routes[route_u].penalty;
                let delta_pen_route_v = self.penalty_excess_load(
                    p,
                    self.routes[route_v].load + p.clients[u_cour].demand - p.clients[v_cour].demand,
                ) - self.routes[route_v].penalty;

                // Quick filter: possibly early elimination of many SWAP* due to the
                // capacity constraints/penalties and bounds on insertion costs
                if delta_pen_route_u
                    + self.nodes[u].delta_removal
                    + delta_pen_route_v
                    + self.nodes[v].delta_removal
                    <= 0.0
                {
                    // Evaluate best reinsertion cost of U in the route of V where V has been removed
                    let (extra_v, best_position_u) =
                        self.get_cheapest_insert_simult_removal(p, u, v);
                    // Evaluate best reinsertion cost of V in the route of U where U has been removed
                    let (extra_u, best_position_v) =
                        self.get_cheapest_insert_simult_removal(p, v, u);

                    // Evaluating final cost
                    let move_cost = delta_pen_route_u
                        + self.nodes[u].delta_removal
                        + extra_u
                        + delta_pen_route_v
                        + self.nodes[v].delta_removal
                        + extra_v
                        + self.penalty_excess_duration(
                            p,
                            self.routes[route_u].duration
                                + self.nodes[u].delta_removal
                                + extra_u
                                + p.clients[v_cour].service_duration
                                - p.clients[u_cour].service_duration,
                        )
                        + self.penalty_excess_duration(
                            p,
                            self.routes[route_v].duration + self.nodes[v].delta_removal + extra_v
                                - p.clients[v_cour].service_duration
                                + p.clients[u_cour].service_duration,
                        );

                    if move_cost < best.move_cost {
                        best = SwapStarElement {
                            move_cost,
                            u: Some(u),
                            best_position_u,
                            v: Some(v),
                            best_position_v,
                        };
                    }
                }
                v = self.nodes[v].next;
            }
            u = self.nodes[u].next;
        }

        // Including RELOCATE from node_u towards route_v (costs nothing to include in the evaluation
        // at this step since we already have the best insertion location).
        // Moreover, since the granularity criterion is different, this can lead to different improving moves.
        let mut u = depot_u_next;
        while !self.nodes[u].is_depot {
            let u_cour = self.nodes[u].cour;
            let bi = &self.best_insert_client[self.bi_index(route_v, u_cour)];
            let best_position_u = bi.best_location[0];
            let delta_dist_route_v = bi.best_cost[0];
            let u_prev_cour = self.nodes[self.nodes[u].prev].cour;
            let u_next_cour = self.nodes[self.nodes[u].next].cour;
            let delta_dist_route_u =
                d(u_prev_cour, u_next_cour) - d(u_prev_cour, u_cour) - d(u_cour, u_next_cour);
            let move_cost = delta_dist_route_u
                + delta_dist_route_v
                + self.penalty_excess_load(p, self.routes[route_u].load - p.clients[u_cour].demand)
                - self.routes[route_u].penalty
                + self.penalty_excess_load(p, self.routes[route_v].load + p.clients[u_cour].demand)
                - self.routes[route_v].penalty
                + self.penalty_excess_duration(
                    p,
                    self.routes[route_u].duration + delta_dist_route_u
                        - p.clients[u_cour].service_duration,
                )
                + self.penalty_excess_duration(
                    p,
                    self.routes[route_v].duration
                        + delta_dist_route_v
                        + p.clients[u_cour].service_duration,
                );

            if move_cost < best.move_cost {
                best = SwapStarElement {
                    move_cost,
                    u: Some(u),
                    best_position_u,
                    v: None,
                    best_position_v: None,
                };
            }
            u = self.nodes[u].next;
        }

        // Including RELOCATE from node_v towards route_u
        let mut v = depot_v_next;
        while !self.nodes[v].is_depot {
            let v_cour = self.nodes[v].cour;
            let bi = &self.best_insert_client[self.bi_index(route_u, v_cour)];
            let best_position_v = bi.best_location[0];
            let delta_dist_route_u = bi.best_cost[0];
            let v_prev_cour = self.nodes[self.nodes[v].prev].cour;
            let v_next_cour = self.nodes[self.nodes[v].next].cour;
            let delta_dist_route_v =
                d(v_prev_cour, v_next_cour) - d(v_prev_cour, v_cour) - d(v_cour, v_next_cour);
            let move_cost = delta_dist_route_u
                + delta_dist_route_v
                + self.penalty_excess_load(p, self.routes[route_u].load + p.clients[v_cour].demand)
                - self.routes[route_u].penalty
                + self.penalty_excess_load(p, self.routes[route_v].load - p.clients[v_cour].demand)
                - self.routes[route_v].penalty
                + self.penalty_excess_duration(
                    p,
                    self.routes[route_u].duration
                        + delta_dist_route_u
                        + p.clients[v_cour].service_duration,
                )
                + self.penalty_excess_duration(
                    p,
                    self.routes[route_v].duration + delta_dist_route_v
                        - p.clients[v_cour].service_duration,
                );

            if move_cost < best.move_cost {
                best = SwapStarElement {
                    move_cost,
                    u: None,
                    best_position_u: None,
                    v: Some(v),
                    best_position_v,
                };
            }
            v = self.nodes[v].next;
        }

        if best.move_cost > -MY_EPSILON {
            return false;
        }

        // Applying the best move in case of improvement
        if let Some(position) = best.best_position_u {
            self.insert_node(
                best.u.expect("U is set together with its position"),
                position,
            );
        }
        if let Some(position) = best.best_position_v {
            self.insert_node(
                best.v.expect("V is set together with its position"),
                position,
            );
        }
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, route_u);
        self.update_route_data(p, route_v);
        true
    }

    /// Calculates the insertion cost and position of U in the route of V, where V is omitted.
    /// Returns (best_cost, best_position).
    fn get_cheapest_insert_simult_removal(
        &self,
        p: &Params,
        u: usize,
        v: usize,
    ) -> (f64, Option<usize>) {
        let best_insert =
            &self.best_insert_client[self.bi_index(self.nodes[v].route, self.nodes[u].cour)];

        // Find the best insertion in the route such that V is not next or pred
        // (the optimal insertion in that case can only belong to the top three locations)
        let mut best_position = best_insert.best_location[0];
        let mut best_cost = best_insert.best_cost[0];
        let mut found = match best_position {
            Some(position) => position != v && self.nodes[position].next != v,
            None => false,
        };
        if !found && best_insert.best_location[1].is_some() {
            best_position = best_insert.best_location[1];
            best_cost = best_insert.best_cost[1];
            let position = best_position.expect("checked above");
            found = position != v && self.nodes[position].next != v;
            if !found && best_insert.best_location[2].is_some() {
                best_position = best_insert.best_location[2];
                best_cost = best_insert.best_cost[2];
                found = true;
            }
        }

        // Also test the insertion in the place of V
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let v_prev = self.nodes[v].prev;
        let v_next = self.nodes[v].next;
        let delta_cost = d(self.nodes[v_prev].cour, self.nodes[u].cour)
            + d(self.nodes[u].cour, self.nodes[v_next].cour)
            - d(self.nodes[v_prev].cour, self.nodes[v_next].cour);
        if !found || delta_cost < best_cost {
            best_position = Some(v_prev);
            best_cost = delta_cost;
        }

        (best_cost, best_position)
    }

    /// Preprocesses all insertion costs of customers of route r1 in route r2.
    fn preprocess_insertions(&mut self, p: &Params, r1: usize, r2: usize) {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut u = self.nodes[self.routes[r1].depot].next;
        while !self.nodes[u].is_depot {
            // Compute the cost of removing U from its route
            let u_cour = self.nodes[u].cour;
            let u_prev_cour = self.nodes[self.nodes[u].prev].cour;
            let u_next = self.nodes[u].next;
            let u_next_cour = self.nodes[u_next].cour;
            self.nodes[u].delta_removal =
                d(u_prev_cour, u_next_cour) - d(u_prev_cour, u_cour) - d(u_cour, u_next_cour);

            // Recompute the table of best insertions in r2 only if it was modified since
            let bi_idx = self.bi_index(r2, u_cour);
            if self.routes[r2].when_last_modified
                > self.best_insert_client[bi_idx].when_last_calculated
            {
                self.best_insert_client[bi_idx].reset();
                self.best_insert_client[bi_idx].when_last_calculated = self.nb_moves;

                let depot2 = self.routes[r2].depot;
                let first = self.nodes[depot2].next;
                let first_cour = self.nodes[first].cour;
                self.best_insert_client[bi_idx].best_cost[0] =
                    d(0, u_cour) + d(u_cour, first_cour) - d(0, first_cour);
                self.best_insert_client[bi_idx].best_location[0] = Some(depot2);

                let mut v = first;
                while !self.nodes[v].is_depot {
                    let v_cour = self.nodes[v].cour;
                    let v_next = self.nodes[v].next;
                    let v_next_cour = self.nodes[v_next].cour;
                    let delta_cost =
                        d(v_cour, u_cour) + d(u_cour, v_next_cour) - d(v_cour, v_next_cour);
                    self.best_insert_client[bi_idx].compare_and_add(delta_cost, v);
                    v = v_next;
                }
            }
            u = u_next;
        }
    }

    /// Solution update: insert U after V.
    fn insert_node(&mut self, u: usize, v: usize) {
        let u_prev = self.nodes[u].prev;
        let u_next = self.nodes[u].next;
        self.nodes[u_prev].next = u_next;
        self.nodes[u_next].prev = u_prev;
        let v_next = self.nodes[v].next;
        self.nodes[v_next].prev = u;
        self.nodes[u].prev = v;
        self.nodes[u].next = v_next;
        self.nodes[v].next = u;
        self.nodes[u].route = self.nodes[v].route;
    }

    /// Solution update: swap U and V.
    fn swap_node(&mut self, u: usize, v: usize) {
        let v_prev = self.nodes[v].prev;
        let v_next = self.nodes[v].next;
        let u_prev = self.nodes[u].prev;
        let u_next = self.nodes[u].next;
        let route_u = self.nodes[u].route;
        let route_v = self.nodes[v].route;

        self.nodes[u_prev].next = v;
        self.nodes[u_next].prev = v;
        self.nodes[v_prev].next = u;
        self.nodes[v_next].prev = u;

        self.nodes[u].prev = v_prev;
        self.nodes[u].next = v_next;
        self.nodes[v].prev = u_prev;
        self.nodes[v].next = u_next;

        self.nodes[u].route = route_v;
        self.nodes[v].route = route_u;
    }

    /// Updates the preprocessed data of a route.
    fn update_route_data(&mut self, p: &Params, route: usize) {
        let mut my_place = 0;
        let mut my_load = 0.0;
        let mut my_time = 0.0;
        let mut my_reversal_distance = 0.0;
        let mut cumulated_x = 0.0;
        let mut cumulated_y = 0.0;

        let depot = self.routes[route].depot;
        {
            let node = &mut self.nodes[depot];
            node.position = 0;
            node.cumulated_load = 0.0;
            node.cumulated_time = 0.0;
            node.cumulated_reversal_distance = 0.0;
        }

        let mut my_node = depot;
        let mut first_it = true;
        while !self.nodes[my_node].is_depot || first_it {
            my_node = self.nodes[my_node].next;
            my_place += 1;
            let cour = self.nodes[my_node].cour;
            let prev_cour = self.nodes[self.nodes[my_node].prev].cour;
            my_load += p.clients[cour].demand;
            my_time += p.time_cost.get(prev_cour, cour) + p.clients[cour].service_duration;
            my_reversal_distance +=
                p.time_cost.get(cour, prev_cour) - p.time_cost.get(prev_cour, cour);
            {
                let node = &mut self.nodes[my_node];
                node.position = my_place;
                node.cumulated_load = my_load;
                node.cumulated_time = my_time;
                node.cumulated_reversal_distance = my_reversal_distance;
            }
            if !self.nodes[my_node].is_depot {
                cumulated_x += p.clients[cour].coord_x;
                cumulated_y += p.clients[cour].coord_y;
                if first_it {
                    self.routes[route]
                        .sector
                        .initialize(p.clients[cour].polar_angle);
                } else {
                    self.routes[route]
                        .sector
                        .extend(p.clients[cour].polar_angle);
                }
            }
            first_it = false;
        }

        let penalty =
            self.penalty_excess_duration(p, my_time) + self.penalty_excess_load(p, my_load);
        let nb_customers = my_place - 1;
        {
            let r = &mut self.routes[route];
            r.duration = my_time;
            r.load = my_load;
            r.penalty = penalty;
            r.nb_customers = nb_customers;
            r.reversal_distance = my_reversal_distance;
            // Remember "when" this route has been last modified
            // (will be used to filter unnecessary move evaluations)
            r.when_last_modified = self.nb_moves;
        }

        if nb_customers == 0 {
            self.routes[route].polar_angle_barycenter = 1.0e30;
            self.empty_routes.insert(route);
        } else {
            self.routes[route].polar_angle_barycenter = (cumulated_y / nb_customers as f64
                - p.clients[0].coord_y)
                .atan2(cumulated_x / nb_customers as f64 - p.clients[0].coord_x);
            self.empty_routes.remove(&route);
        }
    }

    /// Loads an initial solution into the local search structures.
    pub fn load_individual(&mut self, params: &Params, indiv: &Individual) {
        self.empty_routes.clear();
        self.nb_moves = 0;
        for r in 0..self.nb_vehicles {
            let my_depot = self.depot_node(r);
            let my_depot_fin = self.depot_end_node(r);
            self.nodes[my_depot].prev = my_depot_fin;
            self.nodes[my_depot_fin].next = my_depot;
            if !indiv.chrom_r[r].is_empty() {
                let mut my_client = indiv.chrom_r[r][0];
                self.nodes[my_client].route = r;
                self.nodes[my_client].prev = my_depot;
                self.nodes[my_depot].next = my_client;
                for i in 1..indiv.chrom_r[r].len() {
                    let my_client_pred = my_client;
                    my_client = indiv.chrom_r[r][i];
                    self.nodes[my_client].prev = my_client_pred;
                    self.nodes[my_client_pred].next = my_client;
                    self.nodes[my_client].route = r;
                }
                self.nodes[my_client].next = my_depot_fin;
                self.nodes[my_depot_fin].prev = my_client;
            } else {
                self.nodes[my_depot].next = my_depot_fin;
                self.nodes[my_depot_fin].prev = my_depot;
            }
            self.update_route_data(params, r);
            self.routes[r].when_last_tested_swap_star = -1;
            for i in 1..=self.nb_clients {
                // Initializing memory structures
                let bi_idx = self.bi_index(r, i);
                self.best_insert_client[bi_idx].when_last_calculated = -1;
            }
        }

        for i in 1..=self.nb_clients {
            // Initializing memory structures
            self.nodes[i].when_last_tested_ri = -1;
        }
    }

    /// Exports the LS solution into an individual and computes the penalized cost
    /// according to the original penalty weights from Params.
    pub fn export_individual(&self, params: &Params, indiv: &mut Individual) {
        // Empty routes have a polar angle of 1.e30 and therefore always appear at the end
        let mut route_polar_angles: Vec<(f64, usize)> = (0..self.nb_vehicles)
            .map(|r| (self.routes[r].polar_angle_barycenter, r))
            .collect();
        route_polar_angles.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));

        let mut pos = 0;
        for r in 0..self.nb_vehicles {
            indiv.chrom_r[r].clear();
            let mut node = self.nodes[self.routes[route_polar_angles[r].1].depot].next;
            while !self.nodes[node].is_depot {
                indiv.chrom_t[pos] = self.nodes[node].cour;
                indiv.chrom_r[r].push(self.nodes[node].cour);
                node = self.nodes[node].next;
                pos += 1;
            }
        }

        indiv.evaluate_complete_cost(params);
    }
}

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

I need to verify the write-after-read dependency in move8 where `depotVSuiv` is being used right after assignment — the C++ code chains pointer updates together, and I'm checking how my Rust implementation handles this same sequence.

For move9, I'm confirming the else branch logic matches where `depotVFin->prev` gets reassigned and then immediately used to update the next pointer, plus the isDepot case where nodeV becomes the new predecessor of depotVFin.

I'm also verifying that when checking route modification times in the inner loop, I'm reading from the current route of node U at that exact iteration point, not a stale reference.

Now moving on to review the remaining modules: population, genetic, cvrplib, cli, lib, and main.

</details>

核心 LS 写完了。现在写 `population.rs` 和 `genetic.rs`:

<details>
<summary>Tool Call: rust/src/population.rs</summary>

*Status: Completed*

Diff: rust/src/population.rs
```
//! Population management: subpopulations split by feasibility, diversity management,
//! adaptive penalties and best-solution tracking.
//!
//! The C++ version stores `Individual*` and tracks proximities through a multiset of
//! `(distance, Individual*)`. Here every population member gets a unique id at insertion,
//! and proximity lists are sorted vectors of `(distance, id)`. The behavior is identical
//! and deterministic (the C++ ties on pointer addresses, ids play the same role).

use std::collections::VecDeque;
use std::time::Duration;

use crate::algorithm_parameters::AlgorithmParameters;
use crate::individual::Individual;
use crate::local_search::LocalSearch;
use crate::params::Params;
use crate::split::Split;
use crate::util::{format_double, MY_EPSILON};

/// One member of a subpopulation.
struct PopIndiv {
    id: u64,
    indiv: Individual,
    /// Biased fitness of the solution.
    biased_fitness: f64,
    /// Other individuals of the same subpopulation, ordered by increasing proximity.
    proximity: Vec<(f64, u64)>,
}

type SubPopulation = Vec<PopIndiv>;

/// Identifies an individual selected from one of the two subpopulations.
#[derive(Clone, Copy)]
pub struct Selection {
    feasible: bool,
    index: usize,
}

pub struct Population {
    /// Feasible subpopulation, kept ordered by increasing penalized cost.
    feasible_subpop: SubPopulation,
    /// Infeasible subpopulation, kept ordered by increasing penalized cost.
    infeasible_subpop: SubPopulation,
    /// Load feasibility of the recent individuals generated by the LS.
    list_feasibility_load: VecDeque<bool>,
    /// Duration feasibility of the recent individuals generated by the LS.
    list_feasibility_duration: VecDeque<bool>,
    /// Time stamps and values of successive best solutions.
    search_progress: Vec<(Duration, f64)>,
    /// Best solution found during the current restart of the algorithm.
    best_solution_restart: Individual,
    /// Best solution found during the complete execution of the algorithm.
    best_solution_overall: Individual,
    /// Next unique id handed to an inserted individual.
    next_id: u64,
}

impl Population {
    pub fn new(params: &mut Params) -> Self {
        Self {
            feasible_subpop: Vec::new(),
            infeasible_subpop: Vec::new(),
            list_feasibility_load: VecDeque::from(vec![true; params.ap.nb_iter_penalty_management]),
            list_feasibility_duration: VecDeque::from(vec![
                true;
                params.ap.nb_iter_penalty_management
            ]),
            search_progress: Vec::new(),
            best_solution_restart: Individual::new(params),
            best_solution_overall: Individual::new(params),
            next_id: 0,
        }
    }

    /// Creates an initial population of individuals.
    pub fn generate_population(
        &mut self,
        params: &mut Params,
        split: &mut Split,
        local_search: &mut LocalSearch,
    ) {
        if params.verbose {
            println!("----- BUILDING INITIAL POPULATION");
        }
        let mut i = 0;
        while i < 4 * params.ap.mu
            && (i == 0
                || params.ap.time_limit == 0.0
                || params.start_time.elapsed().as_secs_f64() < params.ap.time_limit)
        {
            let mut random_indiv = Individual::new(params);
            split.general_split(params, &mut random_indiv, params.nb_vehicles);
            let (penalty_capacity, penalty_duration) =
                (params.penalty_capacity, params.penalty_duration);
            local_search.run(
                params,
                &mut random_indiv,
                penalty_capacity,
                penalty_duration,
            );
            self.add_individual(&random_indiv, true, params);
            if !random_indiv.eval.is_feasible && params.rng.next() % 2 == 0 {
                // Repair half of the solutions in case of infeasibility
                local_search.run(
                    params,
                    &mut random_indiv,
                    penalty_capacity * 10.0,
                    penalty_duration * 10.0,
                );
                if random_indiv.eval.is_feasible {
                    self.add_individual(&random_indiv, false, params);
                }
            }
            i += 1;
        }
    }

    /// Adds an individual to the population (survivor selection is automatically
    /// triggered whenever the population reaches its maximum size).
    /// Returns true if a new best solution of the run has been found.
    pub fn add_individual(
        &mut self,
        indiv: &Individual,
        update_feasible: bool,
        params: &Params,
    ) -> bool {
        if update_feasible {
            self.list_feasibility_load
                .push_back(indiv.eval.capacity_excess < MY_EPSILON);
            self.list_feasibility_duration
                .push_back(indiv.eval.duration_excess < MY_EPSILON);
            self.list_feasibility_load.pop_front();
            self.list_feasibility_duration.pop_front();
        }

        // Find the adequate subpopulation in relation to the individual feasibility
        let id = self.next_id;
        self.next_id += 1;
        let subpop = if indiv.eval.is_feasible {
            &mut self.feasible_subpop
        } else {
            &mut self.infeasible_subpop
        };

        // Insert a copy of the individual and update the proximity structures
        // by calculating the inter-individual distances
        let mut proximity = Vec::with_capacity(subpop.len() + 1);
        for other in subpop.iter_mut() {
            let distance = broken_pairs_distance(indiv, &other.indiv, params.nb_clients);
            insert_proximity(&mut other.proximity, distance, id);
            insert_proximity(&mut proximity, distance, other.id);
        }

        // Identify the correct location in the subpopulation and insert the individual
        let mut place = subpop.len();
        while place > 0
            && subpop[place - 1].indiv.eval.penalized_cost > indiv.eval.penalized_cost - MY_EPSILON
        {
            place -= 1;
        }
        subpop.insert(
            place,
            PopIndiv {
                id,
                indiv: indiv.clone(),
                biased_fitness: 0.0,
                proximity,
            },
        );

        // Trigger a survivor selection if the maximum subpopulation size is exceeded
        if subpop.len() > params.ap.mu + params.ap.lambda {
            while subpop.len() > params.ap.mu {
                Self::remove_worst_biased_fitness(subpop, &params.ap);
            }
        }

        // Track best solution
        if indiv.eval.is_feasible
            && indiv.eval.penalized_cost
                < self.best_solution_restart.eval.penalized_cost - MY_EPSILON
        {
            self.best_solution_restart.clone_from(indiv); // clone_from reuses the existing buffers
            if indiv.eval.penalized_cost
                < self.best_solution_overall.eval.penalized_cost - MY_EPSILON
            {
                self.best_solution_overall.clone_from(indiv);
                self.search_progress
                    .push((params.start_time.elapsed(), indiv.eval.penalized_cost));
            }
            true
        } else {
            false
        }
    }

    /// Evaluates the biased fitness of all individuals in the subpopulation.
    fn update_biased_fitnesses(pop: &mut SubPopulation, ap: &AlgorithmParameters) {
        // Ranking the individuals based on their diversity contribution
        // (decreasing order of broken-pairs distance)
        let mut ranking: Vec<(f64, usize)> = (0..pop.len())
            .map(|i| {
                (
                    -average_broken_pairs_distance_closest(&pop[i], ap.nb_close),
                    i,
                )
            })
            .collect();
        ranking.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));

        // Updating the biased fitness values
        if pop.len() == 1 {
            pop[0].biased_fitness = 0.0;
        } else {
            let size = pop.len();
            for (rank, &(_, indiv_index)) in ranking.iter().enumerate() {
                // Ranking from 0 to 1
                let div_rank = rank as f64 / (size - 1) as f64;
                let fit_rank = indiv_index as f64 / (size - 1) as f64;
                if size <= ap.nb_elite {
                    // Elite individuals cannot be smaller than population size
                    pop[indiv_index].biased_fitness = fit_rank;
                } else {
                    pop[indiv_index].biased_fitness =
                        fit_rank + (1.0 - ap.nb_elite as f64 / size as f64) * div_rank;
                }
            }
        }
    }

    /// Removes the worst individual in terms of biased fitness.
    fn remove_worst_biased_fitness(pop: &mut SubPopulation, ap: &AlgorithmParameters) {
        Self::update_biased_fitnesses(pop, ap);
        if pop.len() <= 1 {
            panic!("Eliminating the best individual: this should not occur in HGS");
        }

        let mut worst_position = 0;
        let mut is_worst_clone = false;
        let mut worst_biased_fitness = -1.0e30;
        for (i, item) in pop.iter().enumerate().skip(1) {
            // A distance equal to 0 indicates that a clone exists
            let is_clone = average_broken_pairs_distance_closest(item, 1) < MY_EPSILON;
            if (is_clone && !is_worst_clone)
                || (is_clone == is_worst_clone && item.biased_fitness > worst_biased_fitness)
            {
                worst_biased_fitness = item.biased_fitness;
                is_worst_clone = is_clone;
                worst_position = i;
            }
        }

        // Remove the individual from the population and clean its distances
        // from the proximity structures of the other individuals
        let worst = pop.remove(worst_position);
        for other in pop.iter_mut() {
            let position = other
                .proximity
                .iter()
                .position(|&(_, id)| id == worst.id)
                .expect("each member references every other member of its subpopulation");
            other.proximity.remove(position);
        }
    }

    /// Cleans all solutions and generates a new initial population
    /// (only used when running HGS until a time limit).
    pub fn restart(
        &mut self,
        params: &mut Params,
        split: &mut Split,
        local_search: &mut LocalSearch,
    ) {
        if params.verbose {
            println!("----- RESET: CREATING A NEW POPULATION -----");
        }
        self.feasible_subpop.clear();
        self.infeasible_subpop.clear();
        self.best_solution_restart = Individual::new(params);
        self.generate_population(params, split, local_search);
    }

    /// Adaptation of the penalty parameters.
    pub fn manage_penalties(&mut self, params: &mut Params) {
        // Setting some bounds [0.1, 100000] to the penalty values for safety
        let fraction_feasible_load = fraction_true(&self.list_feasibility_load);
        if fraction_feasible_load < params.ap.target_feasible - 0.05
            && params.penalty_capacity < 100000.0
        {
            params.penalty_capacity =
                (params.penalty_capacity * params.ap.penalty_increase).min(100000.0);
        } else if fraction_feasible_load > params.ap.target_feasible + 0.05
            && params.penalty_capacity > 0.1
        {
            params.penalty_capacity =
                (params.penalty_capacity * params.ap.penalty_decrease).max(0.1);
        }

        let fraction_feasible_duration = fraction_true(&self.list_feasibility_duration);
        if fraction_feasible_duration < params.ap.target_feasible - 0.05
            && params.penalty_duration < 100000.0
        {
            params.penalty_duration =
                (params.penalty_duration * params.ap.penalty_increase).min(100000.0);
        } else if fraction_feasible_duration > params.ap.target_feasible + 0.05
            && params.penalty_duration > 0.1
        {
            params.penalty_duration =
                (params.penalty_duration * params.ap.penalty_decrease).max(0.1);
        }

        // Update the evaluations
        for item in self.infeasible_subpop.iter_mut() {
            item.indiv.eval.penalized_cost = item.indiv.eval.distance
                + params.penalty_capacity * item.indiv.eval.capacity_excess
                + params.penalty_duration * item.indiv.eval.duration_excess;
        }

        // If needed, reorder the individuals in the infeasible subpopulation since the
        // penalty values have changed (simple bubble sort for the sake of simplicity)
        let size = self.infeasible_subpop.len();
        for i in 0..size {
            for j in 0..size - i - 1 {
                if self.infeasible_subpop[j].indiv.eval.penalized_cost
                    > self.infeasible_subpop[j + 1].indiv.eval.penalized_cost + MY_EPSILON
                {
                    self.infeasible_subpop.swap(j, j + 1);
                }
            }
        }
    }

    /// Selects an individual by binary tournament in the union of the feasible and
    /// infeasible subpopulations. The returned handle stays valid until the next
    /// mutation of the population.
    pub fn get_binary_tournament(&mut self, params: &mut Params) -> Selection {
        let total_size = self.feasible_subpop.len() + self.infeasible_subpop.len();
        assert!(total_size > 0, "binary tournament on an empty population");

        // Picking two individuals with uniform distribution over the union of subpopulations
        let place1 = params.rng.uniform_below(total_size as u32) as usize;
        let place2 = params.rng.uniform_below(total_size as u32) as usize;

        Self::update_biased_fitnesses(&mut self.feasible_subpop, &params.ap);
        Self::update_biased_fitnesses(&mut self.infeasible_subpop, &params.ap);

        let selection1 = self.resolve_place(place1);
        let selection2 = self.resolve_place(place2);

        // Keeping the best of the two in terms of biased fitness
        if self.member(selection1).biased_fitness < self.member(selection2).biased_fitness {
            selection1
        } else {
            selection2
        }
    }

    fn resolve_place(&self, place: usize) -> Selection {
        if place >= self.feasible_subpop.len() {
            Selection {
                feasible: false,
                index: place - self.feasible_subpop.len(),
            }
        } else {
            Selection {
                feasible: true,
                index: place,
            }
        }
    }

    fn member(&self, selection: Selection) -> &PopIndiv {
        if selection.feasible {
            &self.feasible_subpop[selection.index]
        } else {
            &self.infeasible_subpop[selection.index]
        }
    }

    /// Accesses an individual through a tournament selection handle.
    pub fn individual(&self, selection: Selection) -> &Individual {
        &self.member(selection).indiv
    }

    /// Accesses the best feasible individual.
    pub fn best_feasible(&self) -> Option<&Individual> {
        self.feasible_subpop.first().map(|item| &item.indiv)
    }

    /// Accesses the best infeasible individual.
    pub fn best_infeasible(&self) -> Option<&Individual> {
        self.infeasible_subpop.first().map(|item| &item.indiv)
    }

    /// Accesses the best found solution at all time.
    pub fn best_found(&self) -> Option<&Individual> {
        if self.best_solution_overall.eval.penalized_cost < 1.0e29 {
            Some(&self.best_solution_overall)
        } else {
            None
        }
    }

    /// Prints population state.
    pub fn print_state(&self, nb_iter: usize, nb_iter_no_improvement: usize, params: &Params) {
        if !params.verbose {
            return;
        }
        print!(
            "It {:6} {:6} | T(s) {:.2}",
            nb_iter,
            nb_iter_no_improvement,
            params.start_time.elapsed().as_secs_f64()
        );

        if let Some(best) = self.best_feasible() {
            print!(
                " | Feas {} {:.2} {:.2}",
                self.feasible_subpop.len(),
                best.eval.penalized_cost,
                get_average_cost(&self.feasible_subpop, params.ap.mu)
            );
        } else {
            print!(" | NO-FEASIBLE");
        }

        if let Some(best) = self.best_infeasible() {
            print!(
                " | Inf {} {:.2} {:.2}",
                self.infeasible_subpop.len(),
                best.eval.penalized_cost,
                get_average_cost(&self.infeasible_subpop, params.ap.mu)
            );
        } else {
            print!(" | NO-INFEASIBLE");
        }

        print!(
            " | Div {:.2} {:.2}",
            get_diversity(&self.feasible_subpop, params.ap.mu),
            get_diversity(&self.infeasible_subpop, params.ap.mu)
        );
        print!(
            " | Feas {:.2} {:.2}",
            fraction_true(&self.list_feasibility_load),
            fraction_true(&self.list_feasibility_duration)
        );
        print!(
            " | Pen {:.2} {:.2}",
            params.penalty_capacity, params.penalty_duration
        );
        println!();
    }

    /// Exports in a file the history of solution improvements.
    pub fn export_search_progress(
        &self,
        file_name: &str,
        instance_name: &str,
        seed: u64,
    ) -> std::io::Result<()> {
        let mut output = String::new();
        for (time, value) in &self.search_progress {
            output.push_str(&format!(
                "{};{};{};{}\n",
                instance_name,
                seed,
                format_double(*value),
                format_double(time.as_secs_f64())
            ));
        }
        std::fs::write(file_name, output)
    }
}

/// Distance measure between two individuals, used for diversity calculations.
fn broken_pairs_distance(indiv1: &Individual, indiv2: &Individual, nb_clients: usize) -> f64 {
    let mut differences = 0;
    for j in 1..=nb_clients {
        if indiv1.successors[j] != indiv2.successors[j]
            && indiv1.successors[j] != indiv2.predecessors[j]
        {
            differences += 1;
        }
        if indiv1.predecessors[j] == 0 && indiv2.predecessors[j] != 0 && indiv2.successors[j] != 0 {
            differences += 1;
        }
    }
    differences as f64 / nb_clients as f64
}

/// Inserts (distance, id) in a proximity list kept sorted by (distance, id).
fn insert_proximity(proximity: &mut Vec<(f64, u64)>, distance: f64, id: u64) {
    let position = proximity
        .partition_point(|&(d, other_id)| d < distance || (d == distance && other_id < id));
    proximity.insert(position, (distance, id));
}

/// Returns the average broken pairs distance of this individual with the
/// `nb_closest` individuals of its subpopulation.
fn average_broken_pairs_distance_closest(item: &PopIndiv, nb_closest: usize) -> f64 {
    let max_size = nb_closest.min(item.proximity.len());
    let mut result = 0.0;
    for entry in &item.proximity[..max_size] {
        result += entry.0;
    }
    result / max_size as f64
}

/// Returns the average diversity value among the `mu` best individuals of the subpopulation.
fn get_diversity(pop: &SubPopulation, mu: usize) -> f64 {
    // Only monitoring the "mu" better solutions to avoid too much noise in the measurements
    let size = mu.min(pop.len());
    if size == 0 {
        return -1.0;
    }
    let mut average = 0.0;
    for item in &pop[..size] {
        average += average_broken_pairs_distance_closest(item, size);
    }
    average / size as f64
}

/// Returns the average solution value among the `mu` best individuals of the subpopulation.
fn get_average_cost(pop: &SubPopulation, mu: usize) -> f64 {
    // Only monitoring the "mu" better solutions to avoid too much noise in the measurements
    let size = mu.min(pop.len());
    if size == 0 {
        return -1.0;
    }
    let mut average = 0.0;
    for item in &pop[..size] {
        average += item.indiv.eval.penalized_cost;
    }
    average / size as f64
}

fn fraction_true(list: &VecDeque<bool>) -> f64 {
    list.iter().filter(|&&value| value).count() as f64 / list.len() as f64
}

```

</details>
<details>
<summary>Tool Call: rust/src/genetic.rs</summary>

*Status: Completed*

Diff: rust/src/genetic.rs
```
//! Main procedures of the genetic algorithm, including the OX crossover.

use crate::individual::Individual;
use crate::local_search::LocalSearch;
use crate::params::Params;
use crate::population::Population;
use crate::split::Split;
use crate::util::format_double;

pub struct Genetic {
    pub params: Params,
    split: Split,
    local_search: LocalSearch,
    pub population: Population,
    /// Individual used as scratch space for the crossover result.
    offspring: Individual,
}

impl Genetic {
    pub fn new(mut params: Params) -> Self {
        let split = Split::new(&params);
        let local_search = LocalSearch::new(&params);
        let population = Population::new(&mut params);
        let offspring = Individual::new(&mut params);
        Self {
            params,
            split,
            local_search,
            population,
            offspring,
        }
    }

    /// Runs the genetic algorithm until nb_iter consecutive iterations
    /// without improvement or a time limit.
    pub fn run(&mut self) {
        /* INITIAL POPULATION */
        self.population.generate_population(
            &mut self.params,
            &mut self.split,
            &mut self.local_search,
        );

        let mut nb_iter: usize = 0;
        let mut nb_iter_non_prod: usize = 1;
        if self.params.verbose {
            println!("----- STARTING GENETIC ALGORITHM");
        }
        while nb_iter_non_prod <= self.params.ap.nb_iter
            && (self.params.ap.time_limit == 0.0
                || self.params.start_time.elapsed().as_secs_f64() < self.params.ap.time_limit)
        {
            /* SELECTION AND CROSSOVER */
            let parent1 = self.population.get_binary_tournament(&mut self.params);
            let parent2 = self.population.get_binary_tournament(&mut self.params);
            crossover_ox(
                &mut self.offspring,
                self.population.individual(parent1),
                self.population.individual(parent2),
                &mut self.params,
                &mut self.split,
            );

            /* LOCAL SEARCH */
            let (penalty_capacity, penalty_duration) =
                (self.params.penalty_capacity, self.params.penalty_duration);
            self.local_search.run(
                &mut self.params,
                &mut self.offspring,
                penalty_capacity,
                penalty_duration,
            );
            let mut is_new_best =
                self.population
                    .add_individual(&self.offspring, true, &self.params);
            if !self.offspring.eval.is_feasible && self.params.rng.next() % 2 == 0 {
                // Repair half of the solutions in case of infeasibility
                self.local_search.run(
                    &mut self.params,
                    &mut self.offspring,
                    penalty_capacity * 10.0,
                    penalty_duration * 10.0,
                );
                if self.offspring.eval.is_feasible {
                    is_new_best =
                        self.population
                            .add_individual(&self.offspring, false, &self.params)
                            || is_new_best;
                }
            }

            /* TRACKING THE NUMBER OF ITERATIONS SINCE LAST SOLUTION IMPROVEMENT */
            if is_new_best {
                nb_iter_non_prod = 1;
            } else {
                nb_iter_non_prod += 1;
            }

            /* DIVERSIFICATION, PENALTY MANAGEMENT AND TRACES */
            if nb_iter % self.params.ap.nb_iter_penalty_management == 0 {
                self.population.manage_penalties(&mut self.params);
            }
            if nb_iter % self.params.ap.nb_iter_traces == 0 {
                self.population
                    .print_state(nb_iter, nb_iter_non_prod, &self.params);
            }

            /* FOR TESTS INVOLVING SUCCESSIVE RUNS UNTIL A TIME LIMIT: WE RESET THE ALGORITHM/POPULATION EACH TIME maxIterNonProd IS ATTAINED */
            if self.params.ap.time_limit != 0.0 && nb_iter_non_prod == self.params.ap.nb_iter {
                self.population
                    .restart(&mut self.params, &mut self.split, &mut self.local_search);
                nb_iter_non_prod = 1;
            }

            nb_iter += 1;
        }
        if self.params.verbose {
            println!(
                "----- GENETIC ALGORITHM FINISHED AFTER {} ITERATIONS. TIME SPENT: {}",
                nb_iter,
                format_double(self.params.start_time.elapsed().as_secs_f64())
            );
        }
    }
}

/// OX Crossover: copies a random fragment of parent1 and fills the rest in the
/// order given by parent2, then completes the individual with the Split algorithm.
fn crossover_ox(
    result: &mut Individual,
    parent1: &Individual,
    parent2: &Individual,
    params: &mut Params,
    split: &mut Split,
) {
    let nb_clients = params.nb_clients;

    // Frequency table to track the customers which have been already inserted
    let mut freq_client = vec![false; nb_clients + 1];

    // Picking the beginning and end of the crossover zone
    let start = params.rng.uniform_below(nb_clients as u32) as usize;
    let mut end = params.rng.uniform_below(nb_clients as u32) as usize;

    // Avoid that start and end coincide by accident
    while end == start {
        end = params.rng.uniform_below(nb_clients as u32) as usize;
    }

    // Copy from start to end
    let mut j = start;
    while j % nb_clients != (end + 1) % nb_clients {
        result.chrom_t[j % nb_clients] = parent1.chrom_t[j % nb_clients];
        freq_client[result.chrom_t[j % nb_clients]] = true;
        j += 1;
    }

    // Fill the remaining elements in the order given by the second parent
    for i in 1..=nb_clients {
        let temp = parent2.chrom_t[(end + i) % nb_clients];
        if !freq_client[temp] {
            result.chrom_t[j % nb_clients] = temp;
            j += 1;
        }
    }

    // Complete the individual with the Split algorithm
    split.general_split(params, result, parent1.eval.nb_routes);
}

```

</details>
## Assistant

最后是 `cvrplib.rs`、`cli.rs`、`lib.rs` 和 `main.rs`:

<details>
<summary>Tool Call: rust/src/cvrplib.rs</summary>

*Status: Completed*

Diff: rust/src/cvrplib.rs
```
//! Reader for problem instances following the CVRPLIB conventions.

use crate::matrix::SquareMatrix;

pub struct CvrplibInstance {
    pub x_coords: Vec<f64>,
    pub y_coords: Vec<f64>,
    pub dist_mtx: SquareMatrix,
    pub service_time: Vec<f64>,
    pub demands: Vec<f64>,
    /// Route duration limit.
    pub duration_limit: f64,
    /// Capacity limit.
    pub vehicle_capacity: f64,
    /// Indicates if the problem includes duration constraints.
    pub is_duration_constraint: bool,
    /// Number of clients (excluding the depot).
    pub nb_clients: usize,
}

fn next_token<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<&'a str, String> {
    tokens
        .next()
        .ok_or_else(|| "Unexpected end of input file".to_string())
}

fn parse<T: std::str::FromStr>(token: &str) -> Result<T, String> {
    token
        .parse()
        .map_err(|_| format!("Could not parse value: {}", token))
}

impl CvrplibInstance {
    pub fn read(path: &str, is_rounding_integer: bool) -> Result<CvrplibInstance, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|_| format!("Impossible to open instance file: {}", path))?;

        // The reference implementation skips the first three lines (NAME, COMMENT, TYPE)
        let mut remainder = &content[..];
        for _ in 0..3 {
            match remainder.find('\n') {
                Some(position) => remainder = &remainder[position + 1..],
                None => {
                    remainder = "";
                    break;
                }
            }
        }
        let mut tokens = remainder.split_whitespace();

        // Header section: keyword, separator (":"), value
        let mut nb_clients: Option<usize> = None;
        let mut vehicle_capacity = 1.0e30;
        let mut duration_limit = 1.0e30;
        let mut is_duration_constraint = false;
        let mut service_time_data = 0.0;
        loop {
            let token = next_token(&mut tokens)?;
            match token {
                "NODE_COORD_SECTION" => break,
                "DIMENSION" => {
                    next_token(&mut tokens)?;
                    let dimension: usize = parse(next_token(&mut tokens)?)?;
                    // Need to subtract the depot from the number of nodes
                    nb_clients = Some(dimension.saturating_sub(1));
                }
                "EDGE_WEIGHT_TYPE" => {
                    next_token(&mut tokens)?;
                    next_token(&mut tokens)?;
                }
                "CAPACITY" => {
                    next_token(&mut tokens)?;
                    vehicle_capacity = parse(next_token(&mut tokens)?)?;
                }
                "DISTANCE" => {
                    next_token(&mut tokens)?;
                    duration_limit = parse(next_token(&mut tokens)?)?;
                    is_duration_constraint = true;
                }
                "SERVICE_TIME" => {
                    next_token(&mut tokens)?;
                    service_time_data = parse(next_token(&mut tokens)?)?;
                }
                other => return Err(format!("Unexpected data in input file: {}", other)),
            }
        }
        let nb_clients = match nb_clients {
            Some(n) if n > 0 => n,
            _ => return Err("Number of nodes is undefined".to_string()),
        };
        if vehicle_capacity == 1.0e30 {
            return Err("Vehicle capacity is undefined".to_string());
        }

        // Reading node coordinates: the depot is node 1 in the file (index 0 here),
        // customers are nodes 2, 3, ... (indices 1, 2, ...)
        let mut x_coords = vec![0.0; nb_clients + 1];
        let mut y_coords = vec![0.0; nb_clients + 1];
        for i in 0..=nb_clients {
            let node_number: i64 = parse(next_token(&mut tokens)?)?;
            x_coords[i] = parse(next_token(&mut tokens)?)?;
            y_coords[i] = parse(next_token(&mut tokens)?)?;
            if node_number != i as i64 + 1 {
                return Err("The node numbering is not in order.".to_string());
            }
        }

        // Reading demand information
        let token = next_token(&mut tokens)?;
        if token != "DEMAND_SECTION" {
            return Err(format!("Unexpected data in input file: {}", token));
        }
        let mut demands = vec![0.0; nb_clients + 1];
        let mut service_time = vec![0.0; nb_clients + 1];
        for i in 0..=nb_clients {
            next_token(&mut tokens)?; // node number (not checked, as in the C++ version)
            demands[i] = parse(next_token(&mut tokens)?)?;
            service_time[i] = if i == 0 { 0.0 } else { service_time_data };
        }

        // Calculating the 2D Euclidean distance matrix
        let mut dist_mtx = SquareMatrix::new(nb_clients + 1, 0.0);
        for i in 0..=nb_clients {
            for j in 0..=nb_clients {
                let dx = x_coords[i] - x_coords[j];
                let dy = y_coords[i] - y_coords[j];
                let mut distance = (dx * dx + dy * dy).sqrt();
                if is_rounding_integer {
                    distance = distance.round();
                }
                dist_mtx.set(i, j, distance);
            }
        }

        // Reading depot information (the depot is represented as node 1 in all current instances)
        let token = next_token(&mut tokens)?;
        let depot_index = next_token(&mut tokens)?;
        next_token(&mut tokens)?; // "-1" end marker
        let eof_marker = next_token(&mut tokens)?;
        if token != "DEPOT_SECTION" {
            return Err(format!("Unexpected data in input file: {}", token));
        }
        if depot_index != "1" {
            return Err(format!("Expected depot index 1 instead of {}", depot_index));
        }
        if eof_marker != "EOF" {
            return Err(format!("Unexpected data in input file: {}", eof_marker));
        }

        Ok(CvrplibInstance {
            x_coords,
            y_coords,
            dist_mtx,
            service_time,
            demands,
            duration_limit,
            vehicle_capacity,
            is_duration_constraint,
            nb_clients,
        })
    }
}

```

</details>
<details>
<summary>Tool Call: rust/src/cli.rs</summary>

*Status: Completed*

Diff: rust/src/cli.rs
```
//! Command line parsing for the `hgs` executable.

use crate::algorithm_parameters::AlgorithmParameters;

pub struct CommandLine {
    pub ap: AlgorithmParameters,
    /// Number of vehicles. `None` lets the algorithm compute a reasonable bound.
    pub nb_veh: Option<usize>,
    /// Instance path.
    pub path_instance: String,
    /// Solution path.
    pub path_solution: String,
    pub verbose: bool,
    pub is_rounding_integer: bool,
}

impl CommandLine {
    /// Reads the command line (including the program name in `args[0]`)
    /// and extracts the possible options.
    pub fn parse(args: &[String]) -> Result<CommandLine, String> {
        if args.len() % 2 != 1 || args.len() > 35 || args.len() < 3 {
            println!(
                "----- NUMBER OF COMMANDLINE ARGUMENTS IS INCORRECT: {}",
                args.len()
            );
            display_help();
            return Err("Incorrect line of command".to_string());
        }

        let mut command_line = CommandLine {
            ap: AlgorithmParameters::default(),
            nb_veh: None,
            path_instance: args[1].clone(),
            path_solution: args[2].clone(),
            verbose: true,
            is_rounding_integer: true,
        };

        let mut i = 3;
        while i < args.len() {
            let option = args[i].as_str();
            let value = args[i + 1].as_str();
            let result: Result<(), String> = match option {
                "-t" => parse_into(value, &mut command_line.ap.time_limit),
                "-it" => parse_into(value, &mut command_line.ap.nb_iter),
                "-seed" => parse_into(value, &mut command_line.ap.seed),
                "-veh" => {
                    let veh: usize = parse(value)?;
                    command_line.nb_veh = Some(veh);
                    Ok(())
                }
                "-round" => parse_bool_into(value, &mut command_line.is_rounding_integer),
                "-log" => parse_bool_into(value, &mut command_line.verbose),
                "-nbGranular" => parse_into(value, &mut command_line.ap.nb_granular),
                "-mu" => parse_into(value, &mut command_line.ap.mu),
                "-lambda" => parse_into(value, &mut command_line.ap.lambda),
                "-nbElite" => parse_into(value, &mut command_line.ap.nb_elite),
                "-nbClose" => parse_into(value, &mut command_line.ap.nb_close),
                "-nbIterPenaltyManagement" => {
                    parse_into(value, &mut command_line.ap.nb_iter_penalty_management)
                }
                "-nbIterTraces" => parse_into(value, &mut command_line.ap.nb_iter_traces),
                "-targetFeasible" => parse_into(value, &mut command_line.ap.target_feasible),
                "-penaltyIncrease" => parse_into(value, &mut command_line.ap.penalty_increase),
                "-penaltyDecrease" => parse_into(value, &mut command_line.ap.penalty_decrease),
                _ => {
                    println!("----- ARGUMENT NOT RECOGNIZED: {}", option);
                    display_help();
                    return Err("Incorrect line of command".to_string());
                }
            };
            result?;
            i += 2;
        }

        Ok(command_line)
    }
}

fn parse<T: std::str::FromStr>(value: &str) -> Result<T, String> {
    value
        .parse()
        .map_err(|_| format!("Could not parse argument value: {}", value))
}

fn parse_into<T: std::str::FromStr>(value: &str, target: &mut T) -> Result<(), String> {
    *target = parse(value)?;
    Ok(())
}

fn parse_bool_into(value: &str, target: &mut bool) -> Result<(), String> {
    let as_int: i64 = parse(value)?;
    *target = as_int != 0;
    Ok(())
}

/// Prints information about how to use the code.
pub fn display_help() {
    println!();
    println!("-------------------------------------------------- HGS-CVRP algorithm (2020) ---------------------------------------------------");
    println!("Call with: ./hgs instancePath solPath [-it nbIter] [-t myCPUtime] [-seed mySeed] [-veh nbVehicles] [-log verbose]               ");
    println!("[-it <int>] sets a maximum number of iterations without improvement. Defaults to 20,000                                         ");
    println!("[-t <double>] sets a time limit in seconds. If this parameter is set the code will be run iteratively until the time limit      ");
    println!("[-seed <int>] sets a fixed seed. Defaults to 0                                                                                  ");
    println!("[-veh <int>] sets a prescribed fleet size. Otherwise a reasonable UB on the the fleet size is calculated                        ");
    println!("[-round <bool>] rounding the distance to the nearest integer or not. It can be 0 (not rounding) or 1 (rounding). Defaults to 1. ");
    println!("[-log <bool>] sets the verbose level of the algorithm log. It can be 0 or 1. Defaults to 1.                                     ");
    println!();
    println!("Additional Arguments:                                                                                                           ");
    println!("[-nbIterTraces <int>] Number of iterations between traces display during HGS execution. Defaults to 500                         ");
    println!("[-nbGranular <int>] Granular search parameter, limits the number of moves in the RI local search. Defaults to 20                ");
    println!("[-mu <int>] Minimum population size. Defaults to 25                                                                             ");
    println!("[-lambda <int>] Number of solutions created before reaching the maximum population size (i.e., generation size). Defaults to 40 ");
    println!("[-nbElite <int>] Number of elite individuals. Defaults to 5                                                                     ");
    println!("[-nbClose <int>] Number of closest solutions/individuals considered when calculating diversity contribution. Defaults to 4      ");
    println!("[-nbIterPenaltyManagement <int>] Number of iterations between penalty updates. Defaults to 100                                  ");
    println!("[-targetFeasible <double>] target ratio of feasible individuals between penalty updates. Defaults to 0.2                        ");
    println!("[-penaltyIncrease <double>] penalty increase if insufficient feasible individuals between penalty updates. Defaults to 1.2      ");
    println!("[-penaltyDecrease <double>] penalty decrease if sufficient feasible individuals between penalty updates. Defaults to 0.85       ");
    println!("--------------------------------------------------------------------------------------------------------------------------------");
    println!();
}

```

</details>
<details>
<summary>Tool Call: rust/src/lib.rs</summary>

*Status: Completed*

Diff: rust/src/lib.rs
```
//! HGS-CVRP: A Rust implementation of the Hybrid Genetic Search for the
//! Capacitated Vehicle Routing Problem, with the SWAP* neighborhood.
//!
//! This is a faithful port of the C++ reference implementation by Thibaut Vidal
//! (<https://github.com/vidalt/HGS-CVRP>, MIT license), described in:
//!
//! - Vidal, T., Crainic, T. G., Gendreau, M., Lahrichi, N., Rei, W. (2012).
//!   A hybrid genetic algorithm for multidepot and periodic vehicle routing problems.
//!   Operations Research, 60(3), 611-624.
//! - Vidal, T. (2022). Hybrid genetic search for the CVRP: Open-source implementation
//!   and SWAP* neighborhood. Computers & Operations Research, 140, 105643.
//!
//! # Example
//!
//! ```no_run
//! use hgs_cvrp::{AlgorithmParameters, CvrplibInstance, Genetic, Params};
//!
//! let instance = CvrplibInstance::read("instance.vrp", true).unwrap();
//! let params = Params::new(
//!     &instance.x_coords, &instance.y_coords, instance.dist_mtx,
//!     &instance.service_time, &instance.demands,
//!     instance.vehicle_capacity, instance.duration_limit,
//!     None, instance.is_duration_constraint, true,
//!     AlgorithmParameters::default(),
//! ).unwrap();
//! let mut solver = Genetic::new(params);
//! solver.run();
//! if let Some(best) = solver.population.best_found() {
//!     println!("best cost: {}", best.eval.penalized_cost);
//! }
//! ```

pub mod algorithm_parameters;
pub mod circle_sector;
pub mod cli;
pub mod cvrplib;
pub mod genetic;
pub mod individual;
pub mod local_search;
pub mod matrix;
pub mod params;
pub mod population;
pub mod rng;
pub mod split;
pub mod util;

pub use algorithm_parameters::AlgorithmParameters;
pub use cli::CommandLine;
pub use cvrplib::CvrplibInstance;
pub use genetic::Genetic;
pub use individual::{export_cvrplib_format, EvalIndiv, Individual};
pub use local_search::LocalSearch;
pub use matrix::SquareMatrix;
pub use params::{Client, Params};
pub use population::Population;
pub use split::Split;
pub use util::format_double;

```

</details>
<details>
<summary>Tool Call: rust/src/main.rs</summary>

*Status: Completed*

Diff: rust/src/main.rs
```
use hgs_cvrp::{export_cvrplib_format, CommandLine, CvrplibInstance, Genetic, Params};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(message) = run(&args) {
        println!("EXCEPTION | {}", message);
    }
}

fn run(args: &[String]) -> Result<(), String> {
    // Reading the arguments of the program
    let command_line = CommandLine::parse(args)?;

    // Print all algorithm parameter values
    if command_line.verbose {
        command_line.ap.print();
    }

    // Reading the data file and initializing some data structures
    if command_line.verbose {
        println!("----- READING INSTANCE: {}", command_line.path_instance);
    }
    let instance = CvrplibInstance::read(
        &command_line.path_instance,
        command_line.is_rounding_integer,
    )?;

    let params = Params::new(
        &instance.x_coords,
        &instance.y_coords,
        instance.dist_mtx,
        &instance.service_time,
        &instance.demands,
        instance.vehicle_capacity,
        instance.duration_limit,
        command_line.nb_veh,
        instance.is_duration_constraint,
        command_line.verbose,
        command_line.ap,
    )?;

    // Running HGS
    let mut solver = Genetic::new(params);
    solver.run();

    // Exporting the best solution
    if let Some(best) = solver.population.best_found() {
        if solver.params.verbose {
            println!(
                "----- WRITING BEST SOLUTION IN : {}",
                command_line.path_solution
            );
        }
        if export_cvrplib_format(best, &command_line.path_solution).is_err() {
            println!("----- IMPOSSIBLE TO OPEN: {}", command_line.path_solution);
        }
        let _ = solver.population.export_search_progress(
            &format!("{}.PG.csv", command_line.path_solution),
            &command_line.path_instance,
            solver.params.ap.seed,
        );
    }

    Ok(())
}

```

</details>
## Assistant

所有源码已写完。现在编译并跑单元测试:

<details>
<summary>Tool Call: cargo build --release 2>&1</summary>

*Status: Failed*

`cd` directory "HGS-CVRP/rust" was not in any of the project's worktrees.

</details>
<details>
<summary>Tool Call: cd rust && cargo build --release 2>&1</summary>

*Status: Completed*

Terminal:
```
   Compiling hgs-cvrp v1.0.0 (/home/username/HGS-CVRP/rust)
error[E0689]: can't call method `sqrt` on ambiguous numeric type `{float}`
   --> src/cvrplib.rs:127:56
    |
127 |                 let mut distance = (dx * dx + dy * dy).sqrt();
    |                                                        ^^^^

error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
   --> src/split.rs:191:24
    |
191 |         self.potential[self.idx(0, 0)] = 0.0;
    |         ---------------^^^^-----------
    |         |             ||
    |         |             |immutable borrow occurs here
    |         |             mutable borrow later used here
    |         mutable borrow occurs here
    |
help: try adding a local storing this...
   --> src/split.rs:191:24
    |
191 |         self.potential[self.idx(0, 0)] = 0.0;
    |                        ^^^^^^^^^^^^^^
help: ...and then using that local here
   --> src/split.rs:191:23
    |
191 |         self.potential[self.idx(0, 0)] = 0.0;
    |                       ^^^^^^^^^^^^^^^^

error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
   --> src/split.rs:193:28
    |
193 |             self.potential[self.idx(0, i)] = 1.0e30;
    |             ---------------^^^^-----------
    |             |             ||
    |             |             |immutable borrow occurs here
    |             |             mutable borrow later used here
    |             mutable borrow occurs here
    |
help: try adding a local storing this...
   --> src/split.rs:193:28
    |
193 |             self.potential[self.idx(0, i)] = 1.0e30;
    |                            ^^^^^^^^^^^^^^
help: ...and then using that local here
   --> src/split.rs:193:27
    |
193 |             self.potential[self.idx(0, i)] = 1.0e30;
    |                           ^^^^^^^^^^^^^^^^

error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
   --> src/split.rs:221:40
    |
221 |                         self.potential[self.idx(0, j)] = self.potential[self.idx(0, i)] + cost;
    |                         ---------------^^^^-----------
    |                         |             ||
    |                         |             |immutable borrow occurs here
    |                         |             mutable borrow later used here
    |                         mutable borrow occurs here
    |
help: try adding a local storing this...
   --> src/split.rs:221:40
    |
221 |                         self.potential[self.idx(0, j)] = self.potential[self.idx(0, i)] + cost;
    |                                        ^^^^^^^^^^^^^^
help: ...and then using that local here
   --> src/split.rs:221:39
    |
221 |                         self.potential[self.idx(0, j)] = self.potential[self.idx(0, i)] + cost;
    |                                       ^^^^^^^^^^^^^^^^

error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
   --> src/split.rs:222:35
    |
222 |                         self.pred[self.idx(0, j)] = i;
    |                         ----------^^^^-----------
    |                         |        ||
    |                         |        |immutable borrow occurs here
    |                         |        mutable borrow later used here
    |                         mutable borrow occurs here
    |
help: try adding a local storing this...
   --> src/split.rs:222:35
    |
222 |                         self.pred[self.idx(0, j)] = i;
    |                                   ^^^^^^^^^^^^^^
help: ...and then using that local here
   --> src/split.rs:222:34
    |
222 |                         self.pred[self.idx(0, j)] = i;
    |                                  ^^^^^^^^^^^^^^^^

error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
   --> src/split.rs:231:32
    |
231 | ...   self.potential[self.idx(0, i)] = self.propagate(queue.get_front(), i, 0, params);
    |       ---------------^^^^-----------
    |       |             ||
    |       |             |immutable borrow occurs here
    |       |             mutable borrow later used here
    |       mutable borrow occurs here
    |
help: try adding a local storing this...
   --> src/split.rs:231:32
    |
231 | ...   self.potential[self.idx(0, i)] = self.propagate(queue.get_front(), i, 0, params);
    |                      ^^^^^^^^^^^^^^
help: ...and then using that local here
   --> src/split.rs:231:31
    |
231 | ...   self.potential[self.idx(0, i)] = self.propagate(queue.get_front(), i, 0, params);
    |                     ^^^^^^^^^^^^^^^^

error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
   --> src/split.rs:232:27
    |
232 |                 self.pred[self.idx(0, i)] = queue.get_front();
    |                 ----------^^^^-----------
    |                 |        ||
    |                 |        |immutable borrow occurs here
    |                 |        mutable borrow later used here
    |                 mutable borrow occurs here
    |
help: try adding a local storing this...
   --> src/split.rs:232:27
    |
232 |                 self.pred[self.idx(0, i)] = queue.get_front();
    |                           ^^^^^^^^^^^^^^
help: ...and then using that local here
   --> src/split.rs:232:26
    |
232 |                 self.pred[self.idx(0, i)] = queue.get_front();
    |                          ^^^^^^^^^^^^^^^^

error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
   --> src/split.rs:280:24
    |
280 |         self.potential[self.idx(0, 0)] = 0.0;
    |         ---------------^^^^-----------
    |         |             ||
    |         |             |immutable borrow occurs here
    |         |             mutable borrow later used here
    |         mutable borrow occurs here
    |
help: try adding a local storing this...
   --> src/split.rs:280:24
    |
280 |         self.potential[self.idx(0, 0)] = 0.0;
    |                        ^^^^^^^^^^^^^^
help: ...and then using that local here
   --> src/split.rs:280:23
    |
280 |         self.potential[self.idx(0, 0)] = 0.0;
    |                       ^^^^^^^^^^^^^^^^

error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
   --> src/split.rs:283:32
    |
283 |                 self.potential[self.idx(k, i)] = 1.0e30;
    |                 ---------------^^^^-----------
    |                 |             ||
    |                 |             |immutable borrow occurs here
    |                 |             mutable borrow later used here
    |                 mutable borrow occurs here
    |
help: try adding a local storing this...
   --> src/split.rs:283:32
    |
283 |                 self.potential[self.idx(k, i)] = 1.0e30;
    |                                ^^^^^^^^^^^^^^
help: ...and then using that local here
   --> src/split.rs:283:31
    |
283 |                 self.potential[self.idx(k, i)] = 1.0e30;
    |                               ^^^^^^^^^^^^^^^^

error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
   --> src/split.rs:318:44
    |
318 | ...                   self.potential[self.idx(k + 1, j)] =
    |                       ---------------^^^^---------------
    |                       |             ||
    |                       |             |immutable borrow occurs here
    |                       |             mutable borrow later used here
    |                       mutable borrow occurs here
    |
help: try adding a local storing this...
   --> src/split.rs:318:44
    |
318 | ...                   self.potential[self.idx(k + 1, j)] =
    |                                      ^^^^^^^^^^^^^^^^^^
help: ...and then using that local here
   --> src/split.rs:318:43
    |
318 | ...                   self.potential[self.idx(k + 1, j)] =
    |                                     ^^^^^^^^^^^^^^^^^^^^

error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
   --> src/split.rs:320:39
    |
320 | ...                   self.pred[self.idx(k + 1, j)] = i;
    |                       ----------^^^^---------------
    |                       |        ||
    |                       |        |immutable borrow occurs here
    |                       |        mutable borrow later used here
    |                       mutable borrow occurs here
    |
help: try adding a local storing this...
   --> src/split.rs:320:39
    |
320 | ...                   self.pred[self.idx(k + 1, j)] = i;
    |                                 ^^^^^^^^^^^^^^^^^^
help: ...and then using that local here
   --> src/split.rs:320:38
    |
320 | ...                   self.pred[self.idx(k + 1, j)] = i;
    |                                ^^^^^^^^^^^^^^^^^^^^

error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
   --> src/split.rs:340:36
    |
340 |                     self.potential[self.idx(k + 1, i)] =
    |                     ---------------^^^^---------------
    |                     |             ||
    |                     |             |immutable borrow occurs here
    |                     |             mutable borrow later used here
    |                     mutable borrow occurs here
    |
help: try adding a local storing this...
   --> src/split.rs:340:36
    |
340 |                     self.potential[self.idx(k + 1, i)] =
    |                                    ^^^^^^^^^^^^^^^^^^
help: ...and then using that local here
   --> src/split.rs:340:35
    |
340 |                     self.potential[self.idx(k + 1, i)] =
    |                                   ^^^^^^^^^^^^^^^^^^^^

error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
   --> src/split.rs:342:31
    |
342 |                     self.pred[self.idx(k + 1, i)] = queue.get_front();
    |                     ----------^^^^---------------
    |                     |        ||
    |                     |        |immutable borrow occurs here
    |                     |        mutable borrow later used here
    |                     mutable borrow occurs here
    |
help: try adding a local storing this...
   --> src/split.rs:342:31
    |
342 |                     self.pred[self.idx(k + 1, i)] = queue.get_front();
    |                               ^^^^^^^^^^^^^^^^^^
help: ...and then using that local here
   --> src/split.rs:342:30
    |
342 |                     self.pred[self.idx(k + 1, i)] = queue.get_front();
    |                              ^^^^^^^^^^^^^^^^^^^^

Some errors have detailed explanations: E0502, E0689.
For more information about an error, try `rustc --explain E0502`.
error: could not compile `hgs-cvrp` (lib) due to 13 previous errors

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

I'm running into a borrow checker issue where I'm trying to index into `self.potential` while calling `self.idx()` in the same expression — the mutable borrow of the array conflicts with the immutable borrow needed for the method call. I need to compute the index into a temporary variable first to separate the borrows.遇到了预期的借用检查问题，需要把索引存入局部变量来解决。现在我来查看完整的错误列表。

</details>

<details>
<summary>Tool Call: cd rust && cargo build --release 2>&1 | grep -E "^error"</summary>

*Status: Completed*

Terminal:
```
error[E0689]: can't call method `sqrt` on ambiguous numeric type `{float}`
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
error: could not compile `hgs-cvrp` (lib) due to 13 previous errors

```

</details>
<details>
<summary>Tool Call: cd rust && cargo build --release 2>&1 | grep -E "src/.*\.rs:[0-9]+" | sort -u</summary>

*Status: Completed*

Terminal:
```
   --> src/cvrplib.rs:127:56
   --> src/split.rs:191:23
   --> src/split.rs:191:24
   --> src/split.rs:193:27
   --> src/split.rs:193:28
   --> src/split.rs:221:39
   --> src/split.rs:221:40
   --> src/split.rs:222:34
   --> src/split.rs:222:35
   --> src/split.rs:231:31
   --> src/split.rs:231:32
   --> src/split.rs:232:26
   --> src/split.rs:232:27
   --> src/split.rs:280:23
   --> src/split.rs:280:24
   --> src/split.rs:283:31
   --> src/split.rs:283:32
   --> src/split.rs:318:43
   --> src/split.rs:318:44
   --> src/split.rs:320:38
   --> src/split.rs:320:39
   --> src/split.rs:340:35
   --> src/split.rs:340:36
   --> src/split.rs:342:30
   --> src/split.rs:342:31

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

The errors are concentrated in split.rs with the indexing pattern, and there's a type issue in cvrplib.rs around line 127 where the distance calculation uses parsed float coordinates that might have ambiguous typing.

The fix for cvrplib.rs is to explicitly annotate the x_coords vector as `Vec<f64>` so the type inference chain works properly through the parse call. For split.rs, I'm creating helper methods to encapsulate the potential array access pattern — one for reading values and another for setting them — which will eliminate the indexing ambiguity.

Writing helper methods... I need to systematically go through the code and fix all the assignment sites where I'm running into borrow checker issues, both in split_simple and split_lf.我发现问题集中在 `split.rs` 的索引赋值和 `cvrplib.rs` 的类型推断上，现在逐一修复这些问题。

</details>

<details>
<summary>Tool Call: Read file `rust/src/split.rs` (lines 180-240)</summary>

*Status: Completed*

```HGS-CVRP/rust/src/split.rs
   180	        if !self.split_simple(params, indiv) {
   181	            self.split_lf(params, indiv);
   182	        }
   183	
   184	        // Build up the rest of the Individual structure
   185	        indiv.evaluate_complete_cost(params);
   186	    }
   187	
   188	    /// Split for unlimited fleet. Returns true if the algorithm reached the beginning of the routes.
   189	    fn split_simple(&mut self, params: &Params, indiv: &mut Individual) -> bool {
   190	        // Reinitialize the potential structure
   191	        self.potential[self.idx(0, 0)] = 0.0;
   192	        for i in 1..=params.nb_clients {
   193	            self.potential[self.idx(0, i)] = 1.0e30;
   194	        }
   195	
   196	        // MAIN ALGORITHM -- Simple Split using Bellman's algorithm in topological order.
   197	        // This code has been maintained as it is very simple and can be easily adapted to
   198	        // a variety of constraints, whereas the O(n) Split has a more restricted scope.
   199	        if params.is_duration_constraint {
   200	            for i in 0..params.nb_clients {
   201	                let mut load = 0.0;
   202	                let mut distance = 0.0;
   203	                let mut service_duration = 0.0;
   204	                let mut j = i + 1;
   205	                while j <= params.nb_clients && load <= 1.5 * params.vehicle_capacity {
   206	                    load += self.cli_split[j].demand;
   207	                    service_duration += self.cli_split[j].service_time;
   208	                    if j == i + 1 {
   209	                        distance += self.cli_split[j].d0_x;
   210	                    } else {
   211	                        distance += self.cli_split[j - 1].dnext;
   212	                    }
   213	                    let cost = distance
   214	                        + self.cli_split[j].dx_0
   215	                        + params.penalty_capacity * (load - params.vehicle_capacity).max(0.0)
   216	                        + params.penalty_duration
   217	                            * (distance + self.cli_split[j].dx_0 + service_duration
   218	                                - params.duration_limit)
   219	                                .max(0.0);
   220	                    if self.potential[self.idx(0, i)] + cost < self.potential[self.idx(0, j)] {
   221	                        self.potential[self.idx(0, j)] = self.potential[self.idx(0, i)] + cost;
   222	                        self.pred[self.idx(0, j)] = i;
   223	                    }
   224	                    j += 1;
   225	                }
   226	            }
   227	        } else {
   228	            let mut queue = TrivialDeque::new(params.nb_clients + 1, 0);
   229	            for i in 1..=params.nb_clients {
   230	                // The front is the best predecessor for i
   231	                self.potential[self.idx(0, i)] = self.propagate(queue.get_front(), i, 0, params);
   232	                self.pred[self.idx(0, i)] = queue.get_front();
   233	
   234	                if i < params.nb_clients {
   235	                    // If i is not dominated by the last of the pile
   236	                    if !self.dominates(queue.get_back(), i, 0, params) {
   237	                        // then i will be inserted, need to remove whoever is dominated by i
   238	                        while queue.size() > 0 && self.dominates_right(queue.get_back(), i, 0) {
   239	                            queue.pop_back();
   240	                        }

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

I'm thinking the cleanest approach is to convert `idx` into a standalone function that takes `nb_clients` as a parameter rather than relying on `self`, then compute the stride inline as `nb_clients + 1` and use it to calculate indices with `k * stride + i`. This would let me replace the problematic assignment patterns with local variables instead.I'll fix this by storing the index in a local variable before assignment, then apply the fix across split.rs.

</details>

<details>
<summary>Tool Call: rust/src/split.rs</summary>

*Status: Completed*

Diff: rust/src/split.rs
```
//! Algorithms to decode solutions represented as giant tours into complete CVRP solutions.
//!
//! Direct port of the C++ implementation, itself based on the linear Split of
//! "Vidal, T. (2016). Split algorithm in O(n) for the capacitated vehicle routing problem".

use crate::individual::Individual;
use crate::params::Params;
use crate::util::MY_EPSILON;

#[derive(Clone, Default)]
struct ClientSplit {
    demand: f64,
    service_time: f64,
    d0_x: f64,
    dx_0: f64,
    dnext: f64,
}

/// Simple deque used by the linear Split algorithms.
/// Cursors are signed so that the back can transiently move before the front.
struct TrivialDeque {
    elements: Vec<usize>,
    index_front: i32,
    index_back: i32,
}

impl TrivialDeque {
    fn new(nb_elements: usize, first_node: usize) -> Self {
        let mut elements = vec![0; nb_elements];
        elements[0] = first_node;
        Self {
            elements,
            index_front: 0,
            index_back: 0,
        }
    }

    #[inline]
    fn pop_front(&mut self) {
        self.index_front += 1;
    }

    #[inline]
    fn pop_back(&mut self) {
        self.index_back -= 1;
    }

    #[inline]
    fn push_back(&mut self, i: usize) {
        self.index_back += 1;
        self.elements[self.index_back as usize] = i;
    }

    #[inline]
    fn get_front(&self) -> usize {
        self.elements[self.index_front as usize]
    }

    #[inline]
    fn get_next_front(&self) -> usize {
        self.elements[(self.index_front + 1) as usize]
    }

    #[inline]
    fn get_back(&self) -> usize {
        self.elements[self.index_back as usize]
    }

    fn reset(&mut self, first_node: usize) {
        self.elements[0] = first_node;
        self.index_back = 0;
        self.index_front = 0;
    }

    #[inline]
    fn size(&self) -> i32 {
        self.index_back - self.index_front + 1
    }
}

pub struct Split {
    nb_clients: usize,
    nb_vehicles: usize,
    max_vehicles: usize,

    /* Auxiliary data structures to run the Linear Split algorithm */
    cli_split: Vec<ClientSplit>,
    /// Potential vector, flat (nb_vehicles + 1) x (nb_clients + 1).
    potential: Vec<f64>,
    /// Index of the predecessor in an optimal path, same layout as `potential`.
    pred: Vec<usize>,
    /// sum_distance[i] for i > 1 contains the sum of distances: sum_{k=1}^{i-1} d_{k,k+1}.
    sum_distance: Vec<f64>,
    /// sum_load[i] for i >= 1 contains the sum of loads: sum_{k=1}^{i} q_k.
    sum_load: Vec<f64>,
    /// sum_service[i] for i >= 1 contains the sum of service time: sum_{k=1}^{i} s_k.
    sum_service: Vec<f64>,
}

impl Split {
    pub fn new(params: &Params) -> Self {
        let nb_clients = params.nb_clients;
        let nb_vehicles = params.nb_vehicles;
        Self {
            nb_clients,
            nb_vehicles,
            max_vehicles: 0,
            cli_split: vec![ClientSplit::default(); nb_clients + 1],
            potential: vec![1.0e30; (nb_vehicles + 1) * (nb_clients + 1)],
            pred: vec![0; (nb_vehicles + 1) * (nb_clients + 1)],
            sum_distance: vec![0.0; nb_clients + 1],
            sum_load: vec![0.0; nb_clients + 1],
            sum_service: vec![0.0; nb_clients + 1],
        }
    }

    #[inline]
    fn idx(&self, k: usize, i: usize) -> usize {
        k * (self.nb_clients + 1) + i
    }

    /// Computes the cost of propagating the label i until j (to be called with i < j only).
    #[inline]
    fn propagate(&self, i: usize, j: usize, k: usize, params: &Params) -> f64 {
        self.potential[self.idx(k, i)] + self.sum_distance[j] - self.sum_distance[i + 1]
            + self.cli_split[i + 1].d0_x
            + self.cli_split[j].dx_0
            + params.penalty_capacity
                * (self.sum_load[j] - self.sum_load[i] - params.vehicle_capacity).max(0.0)
    }

    /// Tests if i dominates j as a predecessor for all nodes x >= j + 1 (assuming i < j).
    #[inline]
    fn dominates(&self, i: usize, j: usize, k: usize, params: &Params) -> bool {
        self.potential[self.idx(k, j)] + self.cli_split[j + 1].d0_x
            > self.potential[self.idx(k, i)] + self.cli_split[i + 1].d0_x + self.sum_distance[j + 1]
                - self.sum_distance[i + 1]
                + params.penalty_capacity * (self.sum_load[j] - self.sum_load[i])
    }

    /// Tests if j dominates i as a predecessor for all nodes x >= j + 1 (assuming i < j).
    #[inline]
    fn dominates_right(&self, i: usize, j: usize, k: usize) -> bool {
        self.potential[self.idx(k, j)] + self.cli_split[j + 1].d0_x
            < self.potential[self.idx(k, i)] + self.cli_split[i + 1].d0_x + self.sum_distance[j + 1]
                - self.sum_distance[i + 1]
                + MY_EPSILON
    }

    /// General Split function: tests the unlimited fleet Split first, and only if it
    /// does not produce a feasible solution, runs the Split algorithm for a limited fleet.
    pub fn general_split(
        &mut self,
        params: &Params,
        indiv: &mut Individual,
        nb_max_vehicles: usize,
    ) {
        // Do not apply Split with fewer vehicles than the trivial (LP) bin packing bound
        self.max_vehicles =
            nb_max_vehicles.max((params.total_demand / params.vehicle_capacity).ceil() as usize);

        // Initialization of the data structures for the linear split algorithms
        for i in 1..=params.nb_clients {
            let client = indiv.chrom_t[i - 1];
            self.cli_split[i].demand = params.clients[client].demand;
            self.cli_split[i].service_time = params.clients[client].service_duration;
            self.cli_split[i].d0_x = params.time_cost.get(0, client);
            self.cli_split[i].dx_0 = params.time_cost.get(client, 0);
            self.cli_split[i].dnext = if i < params.nb_clients {
                params.time_cost.get(client, indiv.chrom_t[i])
            } else {
                -1.0e30
            };
            self.sum_load[i] = self.sum_load[i - 1] + self.cli_split[i].demand;
            self.sum_service[i] = self.sum_service[i - 1] + self.cli_split[i].service_time;
            self.sum_distance[i] = self.sum_distance[i - 1] + self.cli_split[i - 1].dnext;
        }

        // We first try the simple split, and then the Split with limited fleet if not successful
        if !self.split_simple(params, indiv) {
            self.split_lf(params, indiv);
        }

        // Build up the rest of the Individual structure
        indiv.evaluate_complete_cost(params);
    }

    /// Split for unlimited fleet. Returns true if the algorithm reached the beginning of the routes.
    fn split_simple(&mut self, params: &Params, indiv: &mut Individual) -> bool {
        // Reinitialize the potential structure
        let origin = self.idx(0, 0);
        self.potential[origin] = 0.0;
        for i in 1..=params.nb_clients {
            let index = self.idx(0, i);
            self.potential[index] = 1.0e30;
        }

        // MAIN ALGORITHM -- Simple Split using Bellman's algorithm in topological order.
        // This code has been maintained as it is very simple and can be easily adapted to
        // a variety of constraints, whereas the O(n) Split has a more restricted scope.
        if params.is_duration_constraint {
            for i in 0..params.nb_clients {
                let mut load = 0.0;
                let mut distance = 0.0;
                let mut service_duration = 0.0;
                let mut j = i + 1;
                while j <= params.nb_clients && load <= 1.5 * params.vehicle_capacity {
                    load += self.cli_split[j].demand;
                    service_duration += self.cli_split[j].service_time;
                    if j == i + 1 {
                        distance += self.cli_split[j].d0_x;
                    } else {
                        distance += self.cli_split[j - 1].dnext;
                    }
                    let cost = distance
                        + self.cli_split[j].dx_0
                        + params.penalty_capacity * (load - params.vehicle_capacity).max(0.0)
                        + params.penalty_duration
                            * (distance + self.cli_split[j].dx_0 + service_duration
                                - params.duration_limit)
                                .max(0.0);
                    if self.potential[self.idx(0, i)] + cost < self.potential[self.idx(0, j)] {
                        let (from, to) = (self.idx(0, i), self.idx(0, j));
                        self.potential[to] = self.potential[from] + cost;
                        self.pred[to] = i;
                    }
                    j += 1;
                }
            }
        } else {
            let mut queue = TrivialDeque::new(params.nb_clients + 1, 0);
            for i in 1..=params.nb_clients {
                // The front is the best predecessor for i
                let index = self.idx(0, i);
                self.potential[index] = self.propagate(queue.get_front(), i, 0, params);
                self.pred[index] = queue.get_front();

                if i < params.nb_clients {
                    // If i is not dominated by the last of the pile
                    if !self.dominates(queue.get_back(), i, 0, params) {
                        // then i will be inserted, need to remove whoever is dominated by i
                        while queue.size() > 0 && self.dominates_right(queue.get_back(), i, 0) {
                            queue.pop_back();
                        }
                        queue.push_back(i);
                    }
                    // Check iteratively if front is dominated by the next front
                    while queue.size() > 1
                        && self.propagate(queue.get_front(), i + 1, 0, params)
                            > self.propagate(queue.get_next_front(), i + 1, 0, params) - MY_EPSILON
                    {
                        queue.pop_front();
                    }
                }
            }
        }

        if self.potential[self.idx(0, params.nb_clients)] > 1.0e29 {
            panic!("ERROR : no Split solution has been propagated until the last node");
        }

        // Filling the chromR structure
        for k in self.max_vehicles..params.nb_vehicles {
            indiv.chrom_r[k].clear();
        }

        let mut end = params.nb_clients;
        for k in (0..self.max_vehicles).rev() {
            indiv.chrom_r[k].clear();
            let begin = self.pred[self.idx(0, end)];
            for ii in begin..end {
                indiv.chrom_r[k].push(indiv.chrom_t[ii]);
            }
            end = begin;
        }

        // Return OK in case the Split algorithm reached the beginning of the routes
        end == 0
    }

    /// Split for limited fleet. Returns true if the algorithm reached the beginning of the routes.
    fn split_lf(&mut self, params: &Params, indiv: &mut Individual) -> bool {
        // Initialize the potential structure
        let origin = self.idx(0, 0);
        self.potential[origin] = 0.0;
        for k in 0..=self.max_vehicles {
            for i in 1..=params.nb_clients {
                let index = self.idx(k, i);
                self.potential[index] = 1.0e30;
            }
        }

        // MAIN ALGORITHM -- Simple Split using Bellman's algorithm in topological order
        if params.is_duration_constraint {
            for k in 0..self.max_vehicles {
                for i in k..params.nb_clients {
                    // The loop stops as soon as the potential is unreachable (interval property)
                    if self.potential[self.idx(k, i)] >= 1.0e29 {
                        break;
                    }
                    let mut load = 0.0;
                    let mut service_duration = 0.0;
                    let mut distance = 0.0;
                    // Setting a maximum limit on load infeasibility to accelerate the algorithm
                    let mut j = i + 1;
                    while j <= params.nb_clients && load <= 1.5 * params.vehicle_capacity {
                        load += self.cli_split[j].demand;
                        service_duration += self.cli_split[j].service_time;
                        if j == i + 1 {
                            distance += self.cli_split[j].d0_x;
                        } else {
                            distance += self.cli_split[j - 1].dnext;
                        }
                        let cost = distance
                            + self.cli_split[j].dx_0
                            + params.penalty_capacity * (load - params.vehicle_capacity).max(0.0)
                            + params.penalty_duration
                                * (distance + self.cli_split[j].dx_0 + service_duration
                                    - params.duration_limit)
                                    .max(0.0);
                        if self.potential[self.idx(k, i)] + cost
                            < self.potential[self.idx(k + 1, j)]
                        {
                            let (from, to) = (self.idx(k, i), self.idx(k + 1, j));
                            self.potential[to] = self.potential[from] + cost;
                            self.pred[to] = i;
                        }
                        j += 1;
                    }
                }
            }
        } else {
            // Without duration constraints in O(n), from "Vidal, T. (2016).
            // Split algorithm in O(n) for the capacitated vehicle routing problem. C&OR"
            let mut queue = TrivialDeque::new(params.nb_clients + 1, 0);
            for k in 0..self.max_vehicles {
                // In the Split problem there is always one feasible solution with k routes
                // that reaches the index k in the tour
                queue.reset(k);

                // The range of potentials < 1.e29 is always an interval.
                // The size of the queue will stay >= 1 until we reach the end of this interval.
                let mut i = k + 1;
                while i <= params.nb_clients && queue.size() > 0 {
                    // The front is the best predecessor for i
                    let index = self.idx(k + 1, i);
                    self.potential[index] = self.propagate(queue.get_front(), i, k, params);
                    self.pred[index] = queue.get_front();

                    if i < params.nb_clients {
                        // If i is not dominated by the last of the pile
                        if !self.dominates(queue.get_back(), i, k, params) {
                            // then i will be inserted, need to remove whoever it dominates
                            while queue.size() > 0 && self.dominates_right(queue.get_back(), i, k) {
                                queue.pop_back();
                            }
                            queue.push_back(i);
                        }

                        // Check iteratively if front is dominated by the next front
                        while queue.size() > 1
                            && self.propagate(queue.get_front(), i + 1, k, params)
                                > self.propagate(queue.get_next_front(), i + 1, k, params)
                                    - MY_EPSILON
                        {
                            queue.pop_front();
                        }
                    }
                    i += 1;
                }
            }
        }

        if self.potential[self.idx(self.max_vehicles, params.nb_clients)] > 1.0e29 {
            panic!("ERROR : no Split solution has been propagated until the last node");
        }

        // It could be cheaper to use a smaller number of vehicles
        let mut min_cost = self.potential[self.idx(self.max_vehicles, params.nb_clients)];
        let mut nb_routes = self.max_vehicles;
        for k in 1..self.max_vehicles {
            if self.potential[self.idx(k, params.nb_clients)] < min_cost {
                min_cost = self.potential[self.idx(k, params.nb_clients)];
                nb_routes = k;
            }
        }

        // Filling the chromR structure
        for k in nb_routes..params.nb_vehicles {
            indiv.chrom_r[k].clear();
        }

        let mut end = params.nb_clients;
        for k in (0..nb_routes).rev() {
            indiv.chrom_r[k].clear();
            let begin = self.pred[self.idx(k + 1, end)];
            for ii in begin..end {
                indiv.chrom_r[k].push(indiv.chrom_t[ii]);
            }
            end = begin;
        }

        // Return OK in case the Split algorithm reached the beginning of the routes
        end == 0
    }
}

```

</details>
<details>
<summary>Tool Call: rust/src/cvrplib.rs</summary>

*Status: Completed*

Diff: rust/src/cvrplib.rs
```
//! Reader for problem instances following the CVRPLIB conventions.

use crate::matrix::SquareMatrix;

pub struct CvrplibInstance {
    pub x_coords: Vec<f64>,
    pub y_coords: Vec<f64>,
    pub dist_mtx: SquareMatrix,
    pub service_time: Vec<f64>,
    pub demands: Vec<f64>,
    /// Route duration limit.
    pub duration_limit: f64,
    /// Capacity limit.
    pub vehicle_capacity: f64,
    /// Indicates if the problem includes duration constraints.
    pub is_duration_constraint: bool,
    /// Number of clients (excluding the depot).
    pub nb_clients: usize,
}

fn next_token<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<&'a str, String> {
    tokens
        .next()
        .ok_or_else(|| "Unexpected end of input file".to_string())
}

fn parse<T: std::str::FromStr>(token: &str) -> Result<T, String> {
    token
        .parse()
        .map_err(|_| format!("Could not parse value: {}", token))
}

impl CvrplibInstance {
    pub fn read(path: &str, is_rounding_integer: bool) -> Result<CvrplibInstance, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|_| format!("Impossible to open instance file: {}", path))?;

        // The reference implementation skips the first three lines (NAME, COMMENT, TYPE)
        let mut remainder = &content[..];
        for _ in 0..3 {
            match remainder.find('\n') {
                Some(position) => remainder = &remainder[position + 1..],
                None => {
                    remainder = "";
                    break;
                }
            }
        }
        let mut tokens = remainder.split_whitespace();

        // Header section: keyword, separator (":"), value
        let mut nb_clients: Option<usize> = None;
        let mut vehicle_capacity = 1.0e30;
        let mut duration_limit = 1.0e30;
        let mut is_duration_constraint = false;
        let mut service_time_data = 0.0;
        loop {
            let token = next_token(&mut tokens)?;
            match token {
                "NODE_COORD_SECTION" => break,
                "DIMENSION" => {
                    next_token(&mut tokens)?;
                    let dimension: usize = parse(next_token(&mut tokens)?)?;
                    // Need to subtract the depot from the number of nodes
                    nb_clients = Some(dimension.saturating_sub(1));
                }
                "EDGE_WEIGHT_TYPE" => {
                    next_token(&mut tokens)?;
                    next_token(&mut tokens)?;
                }
                "CAPACITY" => {
                    next_token(&mut tokens)?;
                    vehicle_capacity = parse(next_token(&mut tokens)?)?;
                }
                "DISTANCE" => {
                    next_token(&mut tokens)?;
                    duration_limit = parse(next_token(&mut tokens)?)?;
                    is_duration_constraint = true;
                }
                "SERVICE_TIME" => {
                    next_token(&mut tokens)?;
                    service_time_data = parse(next_token(&mut tokens)?)?;
                }
                other => return Err(format!("Unexpected data in input file: {}", other)),
            }
        }
        let nb_clients = match nb_clients {
            Some(n) if n > 0 => n,
            _ => return Err("Number of nodes is undefined".to_string()),
        };
        if vehicle_capacity == 1.0e30 {
            return Err("Vehicle capacity is undefined".to_string());
        }

        // Reading node coordinates: the depot is node 1 in the file (index 0 here),
        // customers are nodes 2, 3, ... (indices 1, 2, ...)
        let mut x_coords = vec![0.0; nb_clients + 1];
        let mut y_coords = vec![0.0; nb_clients + 1];
        for i in 0..=nb_clients {
            let node_number: i64 = parse(next_token(&mut tokens)?)?;
            x_coords[i] = parse(next_token(&mut tokens)?)?;
            y_coords[i] = parse(next_token(&mut tokens)?)?;
            if node_number != i as i64 + 1 {
                return Err("The node numbering is not in order.".to_string());
            }
        }

        // Reading demand information
        let token = next_token(&mut tokens)?;
        if token != "DEMAND_SECTION" {
            return Err(format!("Unexpected data in input file: {}", token));
        }
        let mut demands = vec![0.0; nb_clients + 1];
        let mut service_time = vec![0.0; nb_clients + 1];
        for i in 0..=nb_clients {
            next_token(&mut tokens)?; // node number (not checked, as in the C++ version)
            demands[i] = parse(next_token(&mut tokens)?)?;
            service_time[i] = if i == 0 { 0.0 } else { service_time_data };
        }

        // Calculating the 2D Euclidean distance matrix
        let mut dist_mtx = SquareMatrix::new(nb_clients + 1, 0.0);
        for i in 0..=nb_clients {
            for j in 0..=nb_clients {
                let dx: f64 = x_coords[i] - x_coords[j];
                let dy: f64 = y_coords[i] - y_coords[j];
                let mut distance = (dx * dx + dy * dy).sqrt();
                if is_rounding_integer {
                    distance = distance.round();
                }
                dist_mtx.set(i, j, distance);
            }
        }

        // Reading depot information (the depot is represented as node 1 in all current instances)
        let token = next_token(&mut tokens)?;
        let depot_index = next_token(&mut tokens)?;
        next_token(&mut tokens)?; // "-1" end marker
        let eof_marker = next_token(&mut tokens)?;
        if token != "DEPOT_SECTION" {
            return Err(format!("Unexpected data in input file: {}", token));
        }
        if depot_index != "1" {
            return Err(format!("Expected depot index 1 instead of {}", depot_index));
        }
        if eof_marker != "EOF" {
            return Err(format!("Unexpected data in input file: {}", eof_marker));
        }

        Ok(CvrplibInstance {
            x_coords,
            y_coords,
            dist_mtx,
            service_time,
            demands,
            duration_limit,
            vehicle_capacity,
            is_duration_constraint,
            nb_clients,
        })
    }
}

```

</details>
<details>
<summary>Tool Call: cd rust && cargo build --release 2>&1</summary>

*Status: Completed*

Terminal:
```
   Compiling hgs-cvrp v1.0.0 (/home/username/HGS-CVRP/rust)
warning: field `nb_vehicles` is never read
  --> src/split.rs:83:5
   |
81 | pub struct Split {
   |            ----- field in this struct
82 |     nb_clients: usize,
83 |     nb_vehicles: usize,
   |     ^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `hgs-cvrp` (lib) generated 1 warning
    Finished `release` profile [optimized] target(s) in 4.44s

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

The compilation went through with just a dead-code warning, so I'm removing the unused `nb_vehicles` field from Split since I'm already using `params.nb_vehicles` elsewhere and don't need to store it separately.编译通过了，只有一个未使用字段的警告。现在我来清理这个警告并运行单元测试。

</details>

<details>
<summary>Tool Call: rust/src/split.rs</summary>

*Status: Completed*

Diff: rust/src/split.rs
```
//! Algorithms to decode solutions represented as giant tours into complete CVRP solutions.
//!
//! Direct port of the C++ implementation, itself based on the linear Split of
//! "Vidal, T. (2016). Split algorithm in O(n) for the capacitated vehicle routing problem".

use crate::individual::Individual;
use crate::params::Params;
use crate::util::MY_EPSILON;

#[derive(Clone, Default)]
struct ClientSplit {
    demand: f64,
    service_time: f64,
    d0_x: f64,
    dx_0: f64,
    dnext: f64,
}

/// Simple deque used by the linear Split algorithms.
/// Cursors are signed so that the back can transiently move before the front.
struct TrivialDeque {
    elements: Vec<usize>,
    index_front: i32,
    index_back: i32,
}

impl TrivialDeque {
    fn new(nb_elements: usize, first_node: usize) -> Self {
        let mut elements = vec![0; nb_elements];
        elements[0] = first_node;
        Self {
            elements,
            index_front: 0,
            index_back: 0,
        }
    }

    #[inline]
    fn pop_front(&mut self) {
        self.index_front += 1;
    }

    #[inline]
    fn pop_back(&mut self) {
        self.index_back -= 1;
    }

    #[inline]
    fn push_back(&mut self, i: usize) {
        self.index_back += 1;
        self.elements[self.index_back as usize] = i;
    }

    #[inline]
    fn get_front(&self) -> usize {
        self.elements[self.index_front as usize]
    }

    #[inline]
    fn get_next_front(&self) -> usize {
        self.elements[(self.index_front + 1) as usize]
    }

    #[inline]
    fn get_back(&self) -> usize {
        self.elements[self.index_back as usize]
    }

    fn reset(&mut self, first_node: usize) {
        self.elements[0] = first_node;
        self.index_back = 0;
        self.index_front = 0;
    }

    #[inline]
    fn size(&self) -> i32 {
        self.index_back - self.index_front + 1
    }
}

pub struct Split {
    nb_clients: usize,
    max_vehicles: usize,

    /* Auxiliary data structures to run the Linear Split algorithm */
    cli_split: Vec<ClientSplit>,
    /// Potential vector, flat (nb_vehicles + 1) x (nb_clients + 1).
    potential: Vec<f64>,
    /// Index of the predecessor in an optimal path, same layout as `potential`.
    pred: Vec<usize>,
    /// sum_distance[i] for i > 1 contains the sum of distances: sum_{k=1}^{i-1} d_{k,k+1}.
    sum_distance: Vec<f64>,
    /// sum_load[i] for i >= 1 contains the sum of loads: sum_{k=1}^{i} q_k.
    sum_load: Vec<f64>,
    /// sum_service[i] for i >= 1 contains the sum of service time: sum_{k=1}^{i} s_k.
    sum_service: Vec<f64>,
}

impl Split {
    pub fn new(params: &Params) -> Self {
        let nb_clients = params.nb_clients;
        let nb_vehicles = params.nb_vehicles;
        Self {
            nb_clients,
            max_vehicles: 0,
            cli_split: vec![ClientSplit::default(); nb_clients + 1],
            potential: vec![1.0e30; (nb_vehicles + 1) * (nb_clients + 1)],
            pred: vec![0; (nb_vehicles + 1) * (nb_clients + 1)],
            sum_distance: vec![0.0; nb_clients + 1],
            sum_load: vec![0.0; nb_clients + 1],
            sum_service: vec![0.0; nb_clients + 1],
        }
    }

    #[inline]
    fn idx(&self, k: usize, i: usize) -> usize {
        k * (self.nb_clients + 1) + i
    }

    /// Computes the cost of propagating the label i until j (to be called with i < j only).
    #[inline]
    fn propagate(&self, i: usize, j: usize, k: usize, params: &Params) -> f64 {
        self.potential[self.idx(k, i)] + self.sum_distance[j] - self.sum_distance[i + 1]
            + self.cli_split[i + 1].d0_x
            + self.cli_split[j].dx_0
            + params.penalty_capacity
                * (self.sum_load[j] - self.sum_load[i] - params.vehicle_capacity).max(0.0)
    }

    /// Tests if i dominates j as a predecessor for all nodes x >= j + 1 (assuming i < j).
    #[inline]
    fn dominates(&self, i: usize, j: usize, k: usize, params: &Params) -> bool {
        self.potential[self.idx(k, j)] + self.cli_split[j + 1].d0_x
            > self.potential[self.idx(k, i)] + self.cli_split[i + 1].d0_x + self.sum_distance[j + 1]
                - self.sum_distance[i + 1]
                + params.penalty_capacity * (self.sum_load[j] - self.sum_load[i])
    }

    /// Tests if j dominates i as a predecessor for all nodes x >= j + 1 (assuming i < j).
    #[inline]
    fn dominates_right(&self, i: usize, j: usize, k: usize) -> bool {
        self.potential[self.idx(k, j)] + self.cli_split[j + 1].d0_x
            < self.potential[self.idx(k, i)] + self.cli_split[i + 1].d0_x + self.sum_distance[j + 1]
                - self.sum_distance[i + 1]
                + MY_EPSILON
    }

    /// General Split function: tests the unlimited fleet Split first, and only if it
    /// does not produce a feasible solution, runs the Split algorithm for a limited fleet.
    pub fn general_split(
        &mut self,
        params: &Params,
        indiv: &mut Individual,
        nb_max_vehicles: usize,
    ) {
        // Do not apply Split with fewer vehicles than the trivial (LP) bin packing bound
        self.max_vehicles =
            nb_max_vehicles.max((params.total_demand / params.vehicle_capacity).ceil() as usize);

        // Initialization of the data structures for the linear split algorithms
        for i in 1..=params.nb_clients {
            let client = indiv.chrom_t[i - 1];
            self.cli_split[i].demand = params.clients[client].demand;
            self.cli_split[i].service_time = params.clients[client].service_duration;
            self.cli_split[i].d0_x = params.time_cost.get(0, client);
            self.cli_split[i].dx_0 = params.time_cost.get(client, 0);
            self.cli_split[i].dnext = if i < params.nb_clients {
                params.time_cost.get(client, indiv.chrom_t[i])
            } else {
                -1.0e30
            };
            self.sum_load[i] = self.sum_load[i - 1] + self.cli_split[i].demand;
            self.sum_service[i] = self.sum_service[i - 1] + self.cli_split[i].service_time;
            self.sum_distance[i] = self.sum_distance[i - 1] + self.cli_split[i - 1].dnext;
        }

        // We first try the simple split, and then the Split with limited fleet if not successful
        if !self.split_simple(params, indiv) {
            self.split_lf(params, indiv);
        }

        // Build up the rest of the Individual structure
        indiv.evaluate_complete_cost(params);
    }

    /// Split for unlimited fleet. Returns true if the algorithm reached the beginning of the routes.
    fn split_simple(&mut self, params: &Params, indiv: &mut Individual) -> bool {
        // Reinitialize the potential structure
        let origin = self.idx(0, 0);
        self.potential[origin] = 0.0;
        for i in 1..=params.nb_clients {
            let index = self.idx(0, i);
            self.potential[index] = 1.0e30;
        }

        // MAIN ALGORITHM -- Simple Split using Bellman's algorithm in topological order.
        // This code has been maintained as it is very simple and can be easily adapted to
        // a variety of constraints, whereas the O(n) Split has a more restricted scope.
        if params.is_duration_constraint {
            for i in 0..params.nb_clients {
                let mut load = 0.0;
                let mut distance = 0.0;
                let mut service_duration = 0.0;
                let mut j = i + 1;
                while j <= params.nb_clients && load <= 1.5 * params.vehicle_capacity {
                    load += self.cli_split[j].demand;
                    service_duration += self.cli_split[j].service_time;
                    if j == i + 1 {
                        distance += self.cli_split[j].d0_x;
                    } else {
                        distance += self.cli_split[j - 1].dnext;
                    }
                    let cost = distance
                        + self.cli_split[j].dx_0
                        + params.penalty_capacity * (load - params.vehicle_capacity).max(0.0)
                        + params.penalty_duration
                            * (distance + self.cli_split[j].dx_0 + service_duration
                                - params.duration_limit)
                                .max(0.0);
                    if self.potential[self.idx(0, i)] + cost < self.potential[self.idx(0, j)] {
                        let (from, to) = (self.idx(0, i), self.idx(0, j));
                        self.potential[to] = self.potential[from] + cost;
                        self.pred[to] = i;
                    }
                    j += 1;
                }
            }
        } else {
            let mut queue = TrivialDeque::new(params.nb_clients + 1, 0);
            for i in 1..=params.nb_clients {
                // The front is the best predecessor for i
                let index = self.idx(0, i);
                self.potential[index] = self.propagate(queue.get_front(), i, 0, params);
                self.pred[index] = queue.get_front();

                if i < params.nb_clients {
                    // If i is not dominated by the last of the pile
                    if !self.dominates(queue.get_back(), i, 0, params) {
                        // then i will be inserted, need to remove whoever is dominated by i
                        while queue.size() > 0 && self.dominates_right(queue.get_back(), i, 0) {
                            queue.pop_back();
                        }
                        queue.push_back(i);
                    }
                    // Check iteratively if front is dominated by the next front
                    while queue.size() > 1
                        && self.propagate(queue.get_front(), i + 1, 0, params)
                            > self.propagate(queue.get_next_front(), i + 1, 0, params) - MY_EPSILON
                    {
                        queue.pop_front();
                    }
                }
            }
        }

        if self.potential[self.idx(0, params.nb_clients)] > 1.0e29 {
            panic!("ERROR : no Split solution has been propagated until the last node");
        }

        // Filling the chromR structure
        for k in self.max_vehicles..params.nb_vehicles {
            indiv.chrom_r[k].clear();
        }

        let mut end = params.nb_clients;
        for k in (0..self.max_vehicles).rev() {
            indiv.chrom_r[k].clear();
            let begin = self.pred[self.idx(0, end)];
            for ii in begin..end {
                indiv.chrom_r[k].push(indiv.chrom_t[ii]);
            }
            end = begin;
        }

        // Return OK in case the Split algorithm reached the beginning of the routes
        end == 0
    }

    /// Split for limited fleet. Returns true if the algorithm reached the beginning of the routes.
    fn split_lf(&mut self, params: &Params, indiv: &mut Individual) -> bool {
        // Initialize the potential structure
        let origin = self.idx(0, 0);
        self.potential[origin] = 0.0;
        for k in 0..=self.max_vehicles {
            for i in 1..=params.nb_clients {
                let index = self.idx(k, i);
                self.potential[index] = 1.0e30;
            }
        }

        // MAIN ALGORITHM -- Simple Split using Bellman's algorithm in topological order
        if params.is_duration_constraint {
            for k in 0..self.max_vehicles {
                for i in k..params.nb_clients {
                    // The loop stops as soon as the potential is unreachable (interval property)
                    if self.potential[self.idx(k, i)] >= 1.0e29 {
                        break;
                    }
                    let mut load = 0.0;
                    let mut service_duration = 0.0;
                    let mut distance = 0.0;
                    // Setting a maximum limit on load infeasibility to accelerate the algorithm
                    let mut j = i + 1;
                    while j <= params.nb_clients && load <= 1.5 * params.vehicle_capacity {
                        load += self.cli_split[j].demand;
                        service_duration += self.cli_split[j].service_time;
                        if j == i + 1 {
                            distance += self.cli_split[j].d0_x;
                        } else {
                            distance += self.cli_split[j - 1].dnext;
                        }
                        let cost = distance
                            + self.cli_split[j].dx_0
                            + params.penalty_capacity * (load - params.vehicle_capacity).max(0.0)
                            + params.penalty_duration
                                * (distance + self.cli_split[j].dx_0 + service_duration
                                    - params.duration_limit)
                                    .max(0.0);
                        if self.potential[self.idx(k, i)] + cost
                            < self.potential[self.idx(k + 1, j)]
                        {
                            let (from, to) = (self.idx(k, i), self.idx(k + 1, j));
                            self.potential[to] = self.potential[from] + cost;
                            self.pred[to] = i;
                        }
                        j += 1;
                    }
                }
            }
        } else {
            // Without duration constraints in O(n), from "Vidal, T. (2016).
            // Split algorithm in O(n) for the capacitated vehicle routing problem. C&OR"
            let mut queue = TrivialDeque::new(params.nb_clients + 1, 0);
            for k in 0..self.max_vehicles {
                // In the Split problem there is always one feasible solution with k routes
                // that reaches the index k in the tour
                queue.reset(k);

                // The range of potentials < 1.e29 is always an interval.
                // The size of the queue will stay >= 1 until we reach the end of this interval.
                let mut i = k + 1;
                while i <= params.nb_clients && queue.size() > 0 {
                    // The front is the best predecessor for i
                    let index = self.idx(k + 1, i);
                    self.potential[index] = self.propagate(queue.get_front(), i, k, params);
                    self.pred[index] = queue.get_front();

                    if i < params.nb_clients {
                        // If i is not dominated by the last of the pile
                        if !self.dominates(queue.get_back(), i, k, params) {
                            // then i will be inserted, need to remove whoever it dominates
                            while queue.size() > 0 && self.dominates_right(queue.get_back(), i, k) {
                                queue.pop_back();
                            }
                            queue.push_back(i);
                        }

                        // Check iteratively if front is dominated by the next front
                        while queue.size() > 1
                            && self.propagate(queue.get_front(), i + 1, k, params)
                                > self.propagate(queue.get_next_front(), i + 1, k, params)
                                    - MY_EPSILON
                        {
                            queue.pop_front();
                        }
                    }
                    i += 1;
                }
            }
        }

        if self.potential[self.idx(self.max_vehicles, params.nb_clients)] > 1.0e29 {
            panic!("ERROR : no Split solution has been propagated until the last node");
        }

        // It could be cheaper to use a smaller number of vehicles
        let mut min_cost = self.potential[self.idx(self.max_vehicles, params.nb_clients)];
        let mut nb_routes = self.max_vehicles;
        for k in 1..self.max_vehicles {
            if self.potential[self.idx(k, params.nb_clients)] < min_cost {
                min_cost = self.potential[self.idx(k, params.nb_clients)];
                nb_routes = k;
            }
        }

        // Filling the chromR structure
        for k in nb_routes..params.nb_vehicles {
            indiv.chrom_r[k].clear();
        }

        let mut end = params.nb_clients;
        for k in (0..nb_routes).rev() {
            indiv.chrom_r[k].clear();
            let begin = self.pred[self.idx(k + 1, end)];
            for ii in begin..end {
                indiv.chrom_r[k].push(indiv.chrom_t[ii]);
            }
            end = begin;
        }

        // Return OK in case the Split algorithm reached the beginning of the routes
        end == 0
    }
}

```

</details>
<details>
<summary>Tool Call: cd rust && cargo test --release 2>&1</summary>

*Status: Completed*

Terminal:
```
   Compiling hgs-cvrp v1.0.0 (/home/username/HGS-CVRP/rust)
    Finished `release` profile [optimized] target(s) in 6.14s
     Running unittests src/lib.rs (target/release/deps/hgs_cvrp-fa11390886bc7750)

running 5 tests
test circle_sector::tests::enclosure_and_overlap ... ok
test rng::tests::matches_cpp_minstd_rand ... ok
test rng::tests::uniform_below_is_in_range ... ok
test rng::tests::zero_seed_maps_to_one ... ok
test util::tests::formats_like_cpp_ostream ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/hgs-53dfcde4fa2de777)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests hgs_cvrp

running 1 test
test src/lib.rs - (line 15) - compile ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


```

</details>
## Assistant

<details>
<summary>思考过程</summary>

Tests are passing, so now I'm running a smoke test against a real instance while simultaneously building out the C++ reference implementation for comparison.所有测试都通过了。现在我会运行一个快速冒烟测试，同时构建 C++ 参考版本来对比。

</details>

<details>
<summary>Tool Call: cd rust && ./target/release/hgs ../Instances/CVRP/X-n101-k25.vrp /tmp/rust_smoke.sol -seed 1 -it 1000 2>&1</summary>

*Status: Completed*

Terminal:
```
=========== Algorithm Parameters =================
---- nbGranular              is set to 20
---- mu                      is set to 25
---- lambda                  is set to 40
---- nbElite                 is set to 4
---- nbClose                 is set to 5
---- nbIterPenaltyManagement is set to 100
---- targetFeasible          is set to 0.2
---- penaltyDecrease         is set to 0.85
---- penaltyIncrease         is set to 1.2
---- seed                    is set to 1
---- nbIter                  is set to 1000
---- nbIterTraces            is set to 500
---- timeLimit               is set to 0
---- useSwapStar             is set to 1
==================================================
----- READING INSTANCE: ../Instances/CVRP/X-n101-k25.vrp
----- FLEET SIZE WAS NOT SPECIFIED: DEFAULT INITIALIZATION TO 36 VEHICLES
----- INSTANCE SUCCESSFULLY LOADED WITH 100 CLIENTS AND 36 VEHICLES
----- BUILDING INITIAL POPULATION
----- STARTING GENETIC ALGORITHM
It      0      2 | T(s) 0.07 | Feas 28 28156.00 28559.36 | Inf 60 27693.78 28103.65 | Div 0.57 0.50 | Feas 0.00 1.00 | Pen 15.18 0.85
It    500     73 | T(s) 0.32 | Feas 37 27624.00 27746.32 | Inf 25 27553.32 27736.94 | Div 0.36 0.45 | Feas 0.01 1.00 | Pen 37.77 0.38
It   1000    388 | T(s) 0.54 | Feas 60 27595.00 27643.48 | Inf 65 27592.99 27633.19 | Div 0.21 0.18 | Feas 0.14 1.00 | Pen 93.99 0.17
It   1500    888 | T(s) 0.76 | Feas 39 27595.00 27661.40 | Inf 52 27582.12 27650.20 | Div 0.28 0.27 | Feas 0.69 1.00 | Pen 83.12 0.10
----- GENETIC ALGORITHM FINISHED AFTER 1614 ITERATIONS. TIME SPENT: 0.811722
----- WRITING BEST SOLUTION IN : /tmp/rust_smoke.sol

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

The algorithm ran successfully in 0.81 seconds across 1614 iterations, reaching a best feasible solution of 27595 (very close to the optimum of 27591) after just 1000 non-improving iterations. I'm noticing something about the population composition at iteration 0 — it shows 28 feasible and 60 infeasible individuals, which fits within the expected bounds since the maximum population size is mu + lambda = 65 before survivor selection reduces it to 25.

Now I'm going to implement the C++ version and run both algorithms side-by-side using identical settings, then execute the full 20000-iteration test on each to compare their final costs against the known optimum of 27591.烟雾测试通过，性能达到 27595（最优 27591），现在我将构建 C++ 参考版本进行对照。

</details>

<details>
<summary>Tool Call: mkdir -p build && cd build && cmake .. -DCMAKE_BUILD_TYPE=Release -G "Unix Makefiles" > /dev/null 2>&1 && make bin 2>&1 | tail -2</summary>

*Status: Completed*

Terminal:
```
[100%] Linking CXX executable hgs
[100%] Built target bin




```

</details>
## Assistant

<details>
<summary>思考过程</summary>

Now I'm running the official CTest comparisons with the standard configuration settings across the four test instances to validate the C++ implementation against the baseline. I'll execute both binaries sequentially on each case, starting with the first one, and let them run with sufficient timeout since each takes a couple of minutes.现在我将并行运行两个完整的官方测试，先用 C++ 后用 Rust，每个测试默认运行 2 万次无改进迭代。

</details>

<details>
<summary>Tool Call: cd build && time ./hgs ../Instances/CVRP/X-n101-k25.vrp /tmp/cpp_X101.sol -seed 1 -log 0 2>&1; tail -1 /tmp/cpp_X101.sol</summary>

*Status: Completed*

Terminal:
```
9.54user 0.00system 0:11.69elapsed 81%CPU (0avgtext+0avgdata 5680maxresident)k
0inputs+16outputs (0major+487minor)pagefaults 0swaps
Cost 27591



```

</details>
<details>
<summary>Tool Call: cd rust && time ./target/release/hgs ../Instances/CVRP/X-n101-k25.vrp /tmp/rust_X101.sol -seed 1 -log 0 2>&1; tail -1 /tmp/rust_X101.sol</summary>

*Status: Completed*

Terminal:
```
9.56user 0.00system 0:08.76elapsed 109%CPU (0avgtext+0avgdata 4216maxresident)k
0inputs+16outputs (0major+495minor)pagefaults 0swaps
Cost 27591



```

</details>
## Assistant

<details>
<summary>思考过程</summary>

Both implementations found the optimal value of 27591 with nearly identical runtimes, so now I'm testing the other three official benchmarks across C++ and Rust to compare their performance on X-n110-k13, CMT6, and CMT7.两个算法都找到了最优解 27591，运行时间基本相同。现在继续测试其余三个官方用例。

</details>

<details>
<summary>Tool Call: cd build && ./hgs ../Instances/CVRP/X-n110-k13.vrp /tmp/cpp_X110.sol -seed 1 -log 0 2>&1; tail -1 /tmp/cpp_X110.sol; ./hgs ../Instances/CVRP/CMT6.vrp /tmp/cpp_CMT6.sol -seed 1 -round 0 -log 0 2>&1; tail -1 /tmp/cpp_CMT6.sol; ./hgs ../Instances/CVRP/CMT7.vrp /tmp/cpp_CMT7.sol -seed 1 -round 0 -log 0 2>&1; tail -1 /tmp/cpp_CMT7.sol</summary>

*Status: Completed*

Terminal:
```
Cost 14971
Cost 555.43
Cost 909.675



```

</details>
<details>
<summary>Tool Call: cd rust && ./target/release/hgs ../Instances/CVRP/X-n110-k13.vrp /tmp/rust_X110.sol -seed 1 -log 0 2>&1; tail -1 /tmp/rust_X110.sol; ./target/release/hgs ../Instances/CVRP/CMT6.vrp /tmp/rust_CMT6.sol -seed 1 -round 0 -log 0 2>&1; tail -1 /tmp/rust_CMT6.sol; ./target/release/hgs ../Instances/CVRP/CMT7.vrp /tmp/rust_CMT7.sol -seed 1 -round 0 -log 0 2>&1; tail -1 /tmp/rust_CMT7.sol</summary>

*Status: Completed*

Terminal:
```
Cost 14971
Cost 555.43
Cost 909.675



```

</details>
## Assistant

<details>
<summary>思考过程</summary>

All four test cases are passing with exact matches to the expected values, including the two constraint-based instances without rounding. Now I'm running a medium-sized instance with a different seed to do a more comprehensive validation across both implementations.

I also want to verify the output formats match—the trace output, solution file structure, and PG.csv format. Then I'll test a larger instance with time limits and restart behavior by running both with a 5-second timeout and low iteration count to force multiple restarts and confirm they terminate consistently.所有4个官方测试都通过了。现在我来验证输出文件格式,测试时间限制和重启路径,以及一个更大的测试用例。

</details>

<details>
<summary>Tool Call: head -4 /tmp/cpp_X101.sol; echo ===; head -4 /tmp/rust_X101.sol; echo ===; head -3 /tmp/cpp_X101.sol.PG.csv; echo ===; head -3 /tmp/rust_X101.sol.PG.csv</summary>

*Status: Completed*

Terminal:
```
Route #1: 35 46 31
Route #2: 15 22 41 20
Route #3: 54 70 1
Route #4: 92 9 86
===
Route #1: 35 46 31
Route #2: 20 41 22 15
Route #3: 1 70 54
Route #4: 86 9 92
===
../Instances/CVRP/X-n101-k25.vrp;1;28264;0.001648
../Instances/CVRP/X-n101-k25.vrp;1;28222;0.018385
../Instances/CVRP/X-n101-k25.vrp;1;27958;0.071564
===
../Instances/CVRP/X-n101-k25.vrp;1;28156;0.00435142
../Instances/CVRP/X-n101-k25.vrp;1;28033;0.0790815
../Instances/CVRP/X-n101-k25.vrp;1;27965;0.116296

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

The route formats match as expected—even when the paths reverse direction, they represent the same optimal solution found through different search approaches. I'm noticing the C++ timing shows higher precision with six significant digits (0.001648) compared to my result (0.00435142), which suggests a difference in how the output formatting is being handled.

Now I want to test the time-limit restart mechanism on a small instance with tight iteration constraints to force multiple restarts, and also verify the trace output in non-verbose mode before running a larger test case.

</details>

格式一致(路线内容因随机路径不同而异,属预期;两者其实都收敛到同一最优解)。测试时间限制 + 重启逻辑:

<details>
<summary>Tool Call: cd rust && ./target/release/hgs ../Instances/CVRP/X-n101-k25.vrp /tmp/rust_restart.sol -seed 1 -it 300 -t 5 2>&1 | grep -E "RESET|FINISHED|Cost" ; tail -1 /tmp/rust_restart.sol; cd ../build && ./hgs ../Instances/CVRP/X-n101-k25.vrp /tmp/cpp_restart.sol -seed 1 -it 300 -t 5 2>&1 | grep -E "RESET|FINISHED"; tail -1 /tmp/cpp_restart.sol</summary>

*Status: Completed*

Terminal:
```
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- GENETIC ALGORITHM FINISHED AFTER 9672 ITERATIONS. TIME SPENT: 5.00021
Cost 27591
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- RESET: CREATING A NEW POPULATION -----
----- GENETIC ALGORITHM FINISHED AFTER 8526 ITERATIONS. TIME SPENT: 5.00046
Cost 27595

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

Both implementations handle restarts correctly, though the C++ version triggered more restarts in this particular run due to different random trajectories—both behaviors are expected and both converge around the 5-second mark. Now I want to run a larger instance to compare solution quality between Rust and C++ over the same wall time, and also test the duration-constrained mode with restarts enabled to make sure that code path works properly. I've already completed the full CMT6 and CMT7 runs, so next I'll test a medium-sized instance to see how they scale.重启逻辑两边都正常，现在我来对中大型实例做一个质量和性能对比。

</details>

<details>
<summary>Tool Call: ls Instances/CVRP | grep -E "X-n(5|7|10)" | head -8</summary>

*Status: Completed*

Terminal:
```
X-n1001-k43.vrp
X-n101-k25.vrp
X-n106-k14.vrp
X-n502-k39.vrp
X-n513-k21.vrp
X-n524-k153.vrp
X-n536-k96.vrp
X-n548-k50.vrp

```

</details>
<details>
<summary>Tool Call: cd build && time ./hgs ../Instances/CVRP/X-n502-k39.vrp /tmp/cpp_X502.sol -seed 1 -it 2000 -log 0 2>&1; tail -1 /tmp/cpp_X502.sol</summary>

*Status: Completed*

Terminal:
```
75.15user 0.00system 1:14.80elapsed 100%CPU (0avgtext+0avgdata 10212maxresident)k
24inputs+16outputs (0major+1615minor)pagefaults 0swaps
Cost 69320



```

</details>
<details>
<summary>Tool Call: cd rust && time ./target/release/hgs ../Instances/CVRP/X-n502-k39.vrp /tmp/rust_X502.sol -seed 1 -it 2000 -log 0 2>&1; tail -1 /tmp/rust_X502.sol</summary>

*Status: Completed*

Terminal:
```
105.64user 0.00system 1:45.64elapsed 100%CPU (0avgtext+0avgdata 10252maxresident)k
0inputs+16outputs (0major+2015minor)pagefaults 0swaps
Cost 69266



```

</details>
## Assistant

<details>
<summary>思考过程</summary>

Looking at the benchmark results for the larger instance, C++ completes in 75 seconds while Rust takes 105 seconds—a significant 40% slowdown despite comparable solution quality. This is puzzling since they performed equally on the smaller instance, so I'm wondering if the difference stems from bounds checking overhead in tight loops that becomes more pronounced with larger problem sizes and increased cache pressure.

But wait, there's a confounding variable here: the Rust version found a better solution (69266 vs 69320), which means it likely ran more total iterations before hitting the non-improving limit. I need to isolate the per-iteration time by fixing the iteration count for both versions. Let me check the logs to see how many total iterations each actually completed.X-n502 质量更优但耗时更长，两者总迭代数不同。我现在用 FINISHED 行来确认每次迭代的实际耗时。

</details>

<details>
<summary>Tool Call: cd build && ./hgs ../Instances/CVRP/X-n502-k39.vrp /tmp/cpp_X502b.sol -seed 1 -it 2000 2>&1 | grep FINISHED; cd ../rust && ./target/release/hgs ../Instances/CVRP/X-n502-k39.vrp /tmp/rust_X502b.sol -seed 1 -it 2000 2>&1 | grep FINISHED</summary>

*Status: Completed*

Terminal:
```
----- GENETIC ALGORITHM FINISHED AFTER 18901 ITERATIONS. TIME SPENT: 75.9993
----- GENETIC ALGORITHM FINISHED AFTER 26079 ITERATIONS. TIME SPENT: 101.489




```

</details>
## Assistant

<details>
<summary>思考过程</summary>

Looking at the performance metrics, Rust actually has a slight edge in per-iteration speed at 3.89 ms compared to C++'s 4.02 ms, though C++ completed fewer total iterations. The Rust version ran longer and found a better final cost because its random search kept discovering improvements, suggesting the algorithm's convergence behavior differs between the two implementations.

Now I need to verify a few more things: the `-veh` flag for setting fleet size, error handling for invalid instance paths, and CLI help messages. I should also check that the duration-constrained instances (CMT6 and CMT7) are correctly routing through the split duration branch with proper penalty calculations. After that, I'll write up the README and .gitignore, then run clippy to clean up the code.Rust的单次迭代性能略优于C++(3.89 ms vs 4.02 ms),现在我来验证几个边界情况,包括车队大小、错误处理和参数验证。

</details>

<details>
<summary>Tool Call: cd rust && ./target/release/hgs ../Instances/CVRP/X-n101-k25.vrp /tmp/r.sol -seed 1 -it 50 -veh 26 2>&1 | grep -E "FLEET|FINISHED"; ./target/release/hgs /nonexistent.vrp /tmp/r.sol 2>&1 | tail -1; ./target/release/hgs onearg 2>&1 | head -2; echo "exit=$?"</summary>

*Status: Completed*

Terminal:
```
----- FLEET SIZE SPECIFIED: SET TO 26 VEHICLES
----- GENETIC ALGORITHM FINISHED AFTER 148 ITERATIONS. TIME SPENT: 0.163682
EXCEPTION | Impossible to open instance file: /nonexistent.vrp
----- NUMBER OF COMMANDLINE ARGUMENTS IS INCORRECT: 2

exit=0

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

The Rust version handles exceptions the same way as the C++ implementation. I should test the no-swap path with the granular flags, verify that CMT instances with duration constraints work end-to-end, and then document everything with a README for the Rust directory, add a gitignore for the target folder, run diagnostics to check for warnings, and use clippy to catch any linting issues.

Let me check if clippy is available and run it.

</details>

所有边界行为与 C++ 一致。跑一下 clippy 检查代码质量,并查看项目诊断:

<details>
<summary>Tool Call: cd rust && cargo clippy --release 2>&1 | grep -E "^(warning|error)" | sort | uniq -c | sort -rn | head -10</summary>

*Status: Completed*

Terminal:
```
      4 warning: manual implementation of `.is_multiple_of()`
      2 warning: manual `!RangeInclusive::contains` implementation
      1 warning: this looks like you are swapping `self.nodes[xx].next` and `self.nodes[xx].prev` manually
      1 warning: this looks like you are swapping `self.nodes[vv].prev` and `self.nodes[vv].next` manually
      1 warning: this looks like you are swapping `self.nodes[node_num].next` and `self.nodes[node_num].prev` manually
      1 warning: the loop variable `r` is used to index `route_polar_angles`
      1 warning: the loop variable `i` is only used to index `correlated_vertices`
      1 warning: method `next` can be confused for the standard trait method `std::iter::Iterator::next`
      1 warning: manual `rem_euclid` implementation
      1 warning: manual `!Range::contains` implementation

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

I'm keeping certain stylistic clippy lints as-is for traceability with the C++ version, like the modulo check for divisibility and the manual swap operations. For the pointer-reversal loops specifically, using `mem::swap` would require awkward workarounds since I can't easily borrow two mutable references to fields within the same struct element.

Actually, I can simplify the swap operations by getting a mutable reference to the node first, then swapping its next and prev fields directly — that's cleaner and worth applying. For the other lints like `rem_euclid` and the `is_multiple_of` patterns, I'll keep them as intentional choices that mirror the C++ code, though I could add targeted `#[allow(...)]` attributes at the crate level to silence the warnings rather than leaving them as build noise.这些是风格类 lint，我会采纳 `mem::swap` 的建议，其余的保持与 C++ 一致的写法，并在 `lib.rs` 添加 crate 级 allow 注释。

</details>

<details>
<summary>Tool Call: rust/src/local\_search.rs</summary>

*Status: Completed*

Diff: rust/src/local_search.rs
```
//! Local search functions, including the SWAP* neighborhood.
//!
//! The C++ implementation represents the solution as a doubly linked list of `Node*`.
//! Here the nodes live in a single arena (`Vec<Node>`) and links are arena indices,
//! which keeps the same O(1) updates without any reference counting or unsafe code.
//!
//! Arena layout: indices `0..=nb_clients` are the client nodes (0 is a sentinel),
//! then one start depot per route, then one end depot per route.

use std::collections::BTreeSet;

use crate::circle_sector::CircleSector;
use crate::individual::Individual;
use crate::params::Params;
use crate::util::MY_EPSILON;

#[derive(Clone, Default)]
struct Node {
    /// Tells whether this node represents a depot or not.
    is_depot: bool,
    /// Node index (client number, 0 for depots): used for distance matrix lookups.
    cour: usize,
    /// Position in the route.
    position: usize,
    /// "When" the RI moves for this node have been last tested.
    when_last_tested_ri: i32,
    /// Next node in the route order (arena index).
    next: usize,
    /// Previous node in the route order (arena index).
    prev: usize,
    /// Associated route index.
    route: usize,
    /// Cumulated load on this route until the customer (including itself).
    cumulated_load: f64,
    /// Cumulated time on this route until the customer (including itself).
    cumulated_time: f64,
    /// Difference of cost if the segment (0...cour) is reversed (useful for 2-opt asymmetric).
    cumulated_reversal_distance: f64,
    /// Difference of cost in the current route if the node is removed (used in SWAP*).
    delta_removal: f64,
}

#[derive(Clone, Default)]
struct Route {
    /// Route index.
    cour: usize,
    /// Number of customers visited in the route.
    nb_customers: usize,
    /// "When" this route has been last modified.
    when_last_modified: i32,
    /// "When" the SWAP* moves for this route have been last tested.
    when_last_tested_swap_star: i32,
    /// Arena index of the associated start depot.
    depot: usize,
    /// Total time on the route.
    duration: f64,
    /// Total load on the route.
    load: f64,
    /// Difference of cost if the route is reversed.
    reversal_distance: f64,
    /// Current sum of load and duration penalties.
    penalty: f64,
    /// Polar angle of the barycenter of the route.
    polar_angle_barycenter: f64,
    /// Circle sector associated to the set of customers.
    sector: CircleSector,
}

/// Structure used in SWAP* to remember the three best insertion positions
/// of a customer in a given route.
#[derive(Clone)]
struct ThreeBestInsert {
    when_last_calculated: i32,
    best_cost: [f64; 3],
    best_location: [Option<usize>; 3],
}

impl Default for ThreeBestInsert {
    fn default() -> Self {
        Self {
            when_last_calculated: 0,
            best_cost: [1.0e30; 3],
            best_location: [None; 3],
        }
    }
}

impl ThreeBestInsert {
    fn compare_and_add(&mut self, cost_insert: f64, place_insert: usize) {
        if cost_insert >= self.best_cost[2] {
            return;
        }
        if cost_insert >= self.best_cost[1] {
            self.best_cost[2] = cost_insert;
            self.best_location[2] = Some(place_insert);
        } else if cost_insert >= self.best_cost[0] {
            self.best_cost[2] = self.best_cost[1];
            self.best_location[2] = self.best_location[1];
            self.best_cost[1] = cost_insert;
            self.best_location[1] = Some(place_insert);
        } else {
            self.best_cost[2] = self.best_cost[1];
            self.best_location[2] = self.best_location[1];
            self.best_cost[1] = self.best_cost[0];
            self.best_location[1] = self.best_location[0];
            self.best_cost[0] = cost_insert;
            self.best_location[0] = Some(place_insert);
        }
    }

    /// Resets the structure (no insertion calculated).
    fn reset(&mut self) {
        self.best_cost = [1.0e30; 3];
        self.best_location = [None; 3];
    }
}

/// Structure used to keep track of the best SWAP* move.
struct SwapStarElement {
    move_cost: f64,
    u: Option<usize>,
    best_position_u: Option<usize>,
    v: Option<usize>,
    best_position_v: Option<usize>,
}

impl Default for SwapStarElement {
    fn default() -> Self {
        Self {
            move_cost: 1.0e30,
            u: None,
            best_position_u: None,
            v: None,
            best_position_v: None,
        }
    }
}

/// Main local search structure.
pub struct LocalSearch {
    nb_clients: usize,
    nb_vehicles: usize,

    /// Tells whether all moves have been evaluated without success.
    search_completed: bool,
    /// Total number of moves (RI and SWAP*) applied during the local search.
    /// This is not only a counter, it is also used to avoid repeating move evaluations.
    nb_moves: i32,
    /// Randomized order for checking the nodes in the RI local search.
    order_nodes: Vec<usize>,
    /// Randomized order for checking the routes in the SWAP* local search.
    order_routes: Vec<usize>,
    /// Indices of all empty routes.
    empty_routes: BTreeSet<usize>,
    /// Current loop index.
    loop_id: i32,

    /* THE SOLUTION IS REPRESENTED AS A LINKED LIST OF ELEMENTS */
    nodes: Vec<Node>,
    routes: Vec<Route>,
    /// (SWAP*) For each route and node, the cheapest insertion cost, flat layout
    /// `route * (nb_clients + 1) + client`.
    best_insert_client: Vec<ThreeBestInsert>,

    /* TEMPORARY VARIABLES USED IN THE LOCAL SEARCH LOOPS */
    // node_u_prev -> node_u -> node_x -> node_x_next
    // node_v_prev -> node_v -> node_y -> node_y_next
    node_u: usize,
    node_x: usize,
    node_v: usize,
    node_y: usize,
    route_u: usize,
    route_v: usize,
    node_u_prev_index: usize,
    node_u_index: usize,
    node_x_index: usize,
    node_x_next_index: usize,
    node_v_prev_index: usize,
    node_v_index: usize,
    node_y_index: usize,
    node_y_next_index: usize,
    load_u: f64,
    load_x: f64,
    load_v: f64,
    load_y: f64,
    service_u: f64,
    service_x: f64,
    service_v: f64,
    service_y: f64,
    penalty_capacity_ls: f64,
    penalty_duration_ls: f64,
    intra_route_move: bool,
}

impl LocalSearch {
    pub fn new(params: &Params) -> Self {
        let nb_clients = params.nb_clients;
        let nb_vehicles = params.nb_vehicles;

        let mut nodes = vec![Node::default(); nb_clients + 1 + 2 * nb_vehicles];
        for (i, node) in nodes.iter_mut().enumerate().take(nb_clients + 1) {
            node.cour = i;
            node.is_depot = false;
        }
        let mut routes = Vec::with_capacity(nb_vehicles);
        for r in 0..nb_vehicles {
            let depot = nb_clients + 1 + r;
            let depot_end = nb_clients + 1 + nb_vehicles + r;
            nodes[depot].cour = 0;
            nodes[depot].is_depot = true;
            nodes[depot].route = r;
            nodes[depot_end].cour = 0;
            nodes[depot_end].is_depot = true;
            nodes[depot_end].route = r;
            routes.push(Route {
                cour: r,
                depot,
                ..Route::default()
            });
        }

        Self {
            nb_clients,
            nb_vehicles,
            search_completed: false,
            nb_moves: 0,
            order_nodes: (1..=nb_clients).collect(),
            order_routes: (0..nb_vehicles).collect(),
            empty_routes: BTreeSet::new(),
            loop_id: 0,
            nodes,
            routes,
            best_insert_client: vec![ThreeBestInsert::default(); nb_vehicles * (nb_clients + 1)],
            node_u: 0,
            node_x: 0,
            node_v: 0,
            node_y: 0,
            route_u: 0,
            route_v: 0,
            node_u_prev_index: 0,
            node_u_index: 0,
            node_x_index: 0,
            node_x_next_index: 0,
            node_v_prev_index: 0,
            node_v_index: 0,
            node_y_index: 0,
            node_y_next_index: 0,
            load_u: 0.0,
            load_x: 0.0,
            load_v: 0.0,
            load_y: 0.0,
            service_u: 0.0,
            service_x: 0.0,
            service_v: 0.0,
            service_y: 0.0,
            penalty_capacity_ls: 0.0,
            penalty_duration_ls: 0.0,
            intra_route_move: false,
        }
    }

    #[inline]
    fn depot_node(&self, route: usize) -> usize {
        self.nb_clients + 1 + route
    }

    #[inline]
    fn depot_end_node(&self, route: usize) -> usize {
        self.nb_clients + 1 + self.nb_vehicles + route
    }

    #[inline]
    fn bi_index(&self, route: usize, client: usize) -> usize {
        route * (self.nb_clients + 1) + client
    }

    #[inline]
    fn penalty_excess_duration(&self, params: &Params, duration: f64) -> f64 {
        (duration - params.duration_limit).max(0.0) * self.penalty_duration_ls
    }

    #[inline]
    fn penalty_excess_load(&self, params: &Params, load: f64) -> f64 {
        (load - params.vehicle_capacity).max(0.0) * self.penalty_capacity_ls
    }

    /// Runs the local search with the specified penalty values.
    pub fn run(
        &mut self,
        params: &mut Params,
        indiv: &mut Individual,
        penalty_capacity_ls: f64,
        penalty_duration_ls: f64,
    ) {
        self.penalty_capacity_ls = penalty_capacity_ls;
        self.penalty_duration_ls = penalty_duration_ls;
        self.load_individual(params, indiv);

        // Shuffling the order of the nodes explored by the LS to allow for more diversity in the search
        params.rng.shuffle(&mut self.order_nodes);
        params.rng.shuffle(&mut self.order_routes);
        {
            let nb_granular = params.ap.nb_granular as u32;
            let Params {
                rng,
                correlated_vertices,
                ..
            } = params;
            for i in 1..=self.nb_clients {
                // O(n/nbGranular) shuffles on average, to keep linear-time complexity overall
                if rng.next() % nb_granular == 0 {
                    rng.shuffle(&mut correlated_vertices[i]);
                }
            }
        }
        let params: &Params = params;

        self.search_completed = false;
        self.loop_id = 0;
        while !self.search_completed {
            // Allows at least two loops since some moves involving empty routes are not checked at the first loop
            if self.loop_id > 1 {
                self.search_completed = true;
            }

            /* CLASSICAL ROUTE IMPROVEMENT (RI) MOVES SUBJECT TO A PROXIMITY RESTRICTION */
            for pos_u in 0..self.nb_clients {
                self.node_u = self.order_nodes[pos_u];
                let last_test_ri_node_u = self.nodes[self.node_u].when_last_tested_ri;
                self.nodes[self.node_u].when_last_tested_ri = self.nb_moves;
                let u_client = self.nodes[self.node_u].cour;
                'pos_v: for pos_v in 0..params.correlated_vertices[u_client].len() {
                    self.node_v = params.correlated_vertices[u_client][pos_v];
                    let route_u_modified =
                        self.routes[self.nodes[self.node_u].route].when_last_modified;
                    let route_v_modified =
                        self.routes[self.nodes[self.node_v].route].when_last_modified;
                    // Only evaluate moves involving routes that have been modified since
                    // the last move evaluations for node_u
                    if self.loop_id == 0
                        || route_u_modified.max(route_v_modified) > last_test_ri_node_u
                    {
                        // Randomizing the order of the neighborhoods within this loop does not matter much
                        // as we are already randomizing the order of the node pairs (and it's not very common
                        // to find improving moves of different types for the same node pair)
                        self.set_local_variables_route_u(params);
                        self.set_local_variables_route_v(params);
                        if self.move1(params) {
                            continue 'pos_v;
                        } // RELOCATE
                        if self.move2(params) {
                            continue 'pos_v;
                        } // RELOCATE
                        if self.move3(params) {
                            continue 'pos_v;
                        } // RELOCATE
                        if self.node_u_index <= self.node_v_index && self.move4(params) {
                            continue 'pos_v;
                        } // SWAP
                        if self.move5(params) {
                            continue 'pos_v;
                        } // SWAP
                        if self.node_u_index <= self.node_v_index && self.move6(params) {
                            continue 'pos_v;
                        } // SWAP
                        if self.intra_route_move && self.move7(params) {
                            continue 'pos_v;
                        } // 2-OPT
                        if !self.intra_route_move && self.move8(params) {
                            continue 'pos_v;
                        } // 2-OPT*
                        if !self.intra_route_move && self.move9(params) {
                            continue 'pos_v;
                        } // 2-OPT*

                        // Trying moves that insert node_u directly after the depot
                        if self.nodes[self.nodes[self.node_v].prev].is_depot {
                            self.node_v = self.nodes[self.node_v].prev;
                            self.set_local_variables_route_v(params);
                            if self.move1(params) {
                                continue 'pos_v;
                            } // RELOCATE
                            if self.move2(params) {
                                continue 'pos_v;
                            } // RELOCATE
                            if self.move3(params) {
                                continue 'pos_v;
                            } // RELOCATE
                            if !self.intra_route_move && self.move8(params) {
                                continue 'pos_v;
                            } // 2-OPT*
                            if !self.intra_route_move && self.move9(params) {
                                continue 'pos_v;
                            } // 2-OPT*
                        }
                    }
                }

                /* MOVES INVOLVING AN EMPTY ROUTE -- NOT TESTED IN THE FIRST LOOP TO AVOID INCREASING TOO MUCH THE FLEET SIZE */
                if self.loop_id > 0 && !self.empty_routes.is_empty() {
                    let empty_route = *self.empty_routes.iter().next().unwrap();
                    self.node_v = self.routes[empty_route].depot;
                    self.set_local_variables_route_u(params);
                    self.set_local_variables_route_v(params);
                    if self.move1(params) {
                        continue;
                    } // RELOCATE
                    if self.move2(params) {
                        continue;
                    } // RELOCATE
                    if self.move3(params) {
                        continue;
                    } // RELOCATE
                    if self.move9(params) {
                        continue;
                    } // 2-OPT*
                }
            }

            if params.ap.use_swap_star && params.are_coordinates_provided {
                /* (SWAP*) MOVES LIMITED TO ROUTE PAIRS WHOSE CIRCLE SECTORS OVERLAP */
                for r_u in 0..self.nb_vehicles {
                    self.route_u = self.order_routes[r_u];
                    let last_test_swap_star_route_u =
                        self.routes[self.route_u].when_last_tested_swap_star;
                    self.routes[self.route_u].when_last_tested_swap_star = self.nb_moves;
                    for r_v in 0..self.nb_vehicles {
                        self.route_v = self.order_routes[r_v];
                        let route_u = &self.routes[self.route_u];
                        let route_v = &self.routes[self.route_v];
                        if route_u.nb_customers > 0
                            && route_v.nb_customers > 0
                            && route_u.cour < route_v.cour
                            && (self.loop_id == 0
                                || route_u.when_last_modified.max(route_v.when_last_modified)
                                    > last_test_swap_star_route_u)
                            && CircleSector::overlap(&route_u.sector, &route_v.sector)
                        {
                            self.swap_star(params);
                        }
                    }
                }
            }

            self.loop_id += 1;
        }

        // Register the solution produced by the LS in the individual
        self.export_individual(params, indiv);
    }

    fn set_local_variables_route_u(&mut self, params: &Params) {
        self.route_u = self.nodes[self.node_u].route;
        self.node_x = self.nodes[self.node_u].next;
        self.node_x_next_index = self.nodes[self.nodes[self.node_x].next].cour;
        self.node_u_index = self.nodes[self.node_u].cour;
        self.node_u_prev_index = self.nodes[self.nodes[self.node_u].prev].cour;
        self.node_x_index = self.nodes[self.node_x].cour;
        self.load_u = params.clients[self.node_u_index].demand;
        self.service_u = params.clients[self.node_u_index].service_duration;
        self.load_x = params.clients[self.node_x_index].demand;
        self.service_x = params.clients[self.node_x_index].service_duration;
    }

    fn set_local_variables_route_v(&mut self, params: &Params) {
        self.route_v = self.nodes[self.node_v].route;
        self.node_y = self.nodes[self.node_v].next;
        self.node_y_next_index = self.nodes[self.nodes[self.node_y].next].cour;
        self.node_v_index = self.nodes[self.node_v].cour;
        self.node_v_prev_index = self.nodes[self.nodes[self.node_v].prev].cour;
        self.node_y_index = self.nodes[self.node_y].cour;
        self.load_v = params.clients[self.node_v_index].demand;
        self.service_v = params.clients[self.node_v_index].service_duration;
        self.load_y = params.clients[self.node_y_index].demand;
        self.service_y = params.clients[self.node_y_index].service_duration;
        self.intra_route_move = self.route_u == self.route_v;
    }

    /// If U is a client node, remove U and insert it after V.
    fn move1(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_x_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_u_index, self.node_x_index);
        let mut cost_supp_v = d(self.node_v_index, self.node_u_index)
            + d(self.node_u_index, self.node_y_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            // Early move pruning to save CPU time: this move cannot improve
            // without checking additional (load, duration...) constraints
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self
                .penalty_excess_duration(p, route_u.duration + cost_supp_u - self.service_u)
                + self.penalty_excess_load(p, route_u.load - self.load_u)
                - route_u.penalty;

            cost_supp_v += self
                .penalty_excess_duration(p, route_v.duration + cost_supp_v + self.service_u)
                + self.penalty_excess_load(p, route_v.load + self.load_u)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u_index == self.node_y_index {
            return false;
        }

        self.insert_node(self.node_u, self.node_v);
        self.nb_moves += 1; // Increment move counter before updating route data
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If U and X are client nodes, remove them and insert (U,X) after V.
    fn move2(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_x_next_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_x_index, self.node_x_next_index);
        let mut cost_supp_v = d(self.node_v_index, self.node_u_index)
            + d(self.node_x_index, self.node_y_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u
                    - d(self.node_u_index, self.node_x_index)
                    - self.service_u
                    - self.service_x,
            ) + self
                .penalty_excess_load(p, route_u.load - self.load_u - self.load_x)
                - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration
                    + cost_supp_v
                    + d(self.node_u_index, self.node_x_index)
                    + self.service_u
                    + self.service_x,
            ) + self
                .penalty_excess_load(p, route_v.load + self.load_u + self.load_x)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u == self.node_y
            || self.node_v == self.node_x
            || self.nodes[self.node_x].is_depot
        {
            return false;
        }

        self.insert_node(self.node_u, self.node_v);
        self.insert_node(self.node_x, self.node_u);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If U and X are client nodes, remove them and insert (X,U) after V.
    fn move3(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_x_next_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_u_index, self.node_x_index)
            - d(self.node_x_index, self.node_x_next_index);
        let mut cost_supp_v = d(self.node_v_index, self.node_x_index)
            + d(self.node_x_index, self.node_u_index)
            + d(self.node_u_index, self.node_y_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u - self.service_u - self.service_x,
            ) + self
                .penalty_excess_load(p, route_u.load - self.load_u - self.load_x)
                - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration + cost_supp_v + self.service_u + self.service_x,
            ) + self
                .penalty_excess_load(p, route_v.load + self.load_u + self.load_x)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u == self.node_y
            || self.node_x == self.node_v
            || self.nodes[self.node_x].is_depot
        {
            return false;
        }

        self.insert_node(self.node_x, self.node_v);
        self.insert_node(self.node_u, self.node_x);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If U and V are client nodes, swap U and V.
    fn move4(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_v_index)
            + d(self.node_v_index, self.node_x_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_u_index, self.node_x_index);
        let mut cost_supp_v = d(self.node_v_prev_index, self.node_u_index)
            + d(self.node_u_index, self.node_y_index)
            - d(self.node_v_prev_index, self.node_v_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u + self.service_v - self.service_u,
            ) + self
                .penalty_excess_load(p, route_u.load + self.load_v - self.load_u)
                - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration + cost_supp_v - self.service_v + self.service_u,
            ) + self
                .penalty_excess_load(p, route_v.load + self.load_u - self.load_v)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u_index == self.node_v_prev_index || self.node_u_index == self.node_y_index {
            return false;
        }

        self.swap_node(self.node_u, self.node_v);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If U, X and V are client nodes, swap (U,X) and V.
    fn move5(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_v_index)
            + d(self.node_v_index, self.node_x_next_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_x_index, self.node_x_next_index);
        let mut cost_supp_v = d(self.node_v_prev_index, self.node_u_index)
            + d(self.node_x_index, self.node_y_index)
            - d(self.node_v_prev_index, self.node_v_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u - d(self.node_u_index, self.node_x_index)
                    + self.service_v
                    - self.service_u
                    - self.service_x,
            ) + self
                .penalty_excess_load(p, route_u.load + self.load_v - self.load_u - self.load_x)
                - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration + cost_supp_v + d(self.node_u_index, self.node_x_index)
                    - self.service_v
                    + self.service_u
                    + self.service_x,
            ) + self
                .penalty_excess_load(p, route_v.load + self.load_u + self.load_x - self.load_v)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u == self.nodes[self.node_v].prev
            || self.node_x == self.nodes[self.node_v].prev
            || self.node_u == self.node_y
            || self.nodes[self.node_x].is_depot
        {
            return false;
        }

        self.swap_node(self.node_u, self.node_v);
        self.insert_node(self.node_x, self.node_u);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If (U,X) and (V,Y) are client nodes, swap (U,X) and (V,Y).
    fn move6(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_v_index)
            + d(self.node_y_index, self.node_x_next_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_x_index, self.node_x_next_index);
        let mut cost_supp_v = d(self.node_v_prev_index, self.node_u_index)
            + d(self.node_x_index, self.node_y_next_index)
            - d(self.node_v_prev_index, self.node_v_index)
            - d(self.node_y_index, self.node_y_next_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u - d(self.node_u_index, self.node_x_index)
                    + d(self.node_v_index, self.node_y_index)
                    + self.service_v
                    + self.service_y
                    - self.service_u
                    - self.service_x,
            ) + self.penalty_excess_load(
                p,
                route_u.load + self.load_v + self.load_y - self.load_u - self.load_x,
            ) - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration + cost_supp_v + d(self.node_u_index, self.node_x_index)
                    - d(self.node_v_index, self.node_y_index)
                    - self.service_v
                    - self.service_y
                    + self.service_u
                    + self.service_x,
            ) + self.penalty_excess_load(
                p,
                route_v.load + self.load_u + self.load_x - self.load_v - self.load_y,
            ) - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.nodes[self.node_x].is_depot
            || self.nodes[self.node_y].is_depot
            || self.node_y == self.nodes[self.node_u].prev
            || self.node_u == self.node_y
            || self.node_x == self.node_v
            || self.node_v == self.nodes[self.node_x].next
        {
            return false;
        }

        self.swap_node(self.node_u, self.node_v);
        self.swap_node(self.node_x, self.node_y);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If route(U) == route(V), replace (U,X) and (V,Y) by (U,V) and (X,Y).
    fn move7(&mut self, p: &Params) -> bool {
        if self.nodes[self.node_u].position > self.nodes[self.node_v].position {
            return false;
        }

        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let cost = d(self.node_u_index, self.node_v_index)
            + d(self.node_x_index, self.node_y_index)
            - d(self.node_u_index, self.node_x_index)
            - d(self.node_v_index, self.node_y_index)
            + self.nodes[self.node_v].cumulated_reversal_distance
            - self.nodes[self.node_x].cumulated_reversal_distance;

        if cost > -MY_EPSILON {
            return false;
        }
        if self.nodes[self.node_u].next == self.node_v {
            return false;
        }

        // Reverse the segment between X and V
        let mut node_num = self.nodes[self.node_x].next;
        self.nodes[self.node_x].prev = node_num;
        self.nodes[self.node_x].next = self.node_y;

        while node_num != self.node_v {
            let temp = self.nodes[node_num].next;
            let node = &mut self.nodes[node_num];
            std::mem::swap(&mut node.next, &mut node.prev);
            node_num = temp;
        }

        self.nodes[self.node_v].next = self.nodes[self.node_v].prev;
        self.nodes[self.node_v].prev = self.node_u;
        self.nodes[self.node_u].next = self.node_v;
        self.nodes[self.node_y].prev = self.node_x;

        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        true
    }

    /// If route(U) != route(V), replace (U,X) and (V,Y) by (U,V) and (X,Y).
    fn move8(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost = {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            d(self.node_u_index, self.node_v_index) + d(self.node_x_index, self.node_y_index)
                - d(self.node_u_index, self.node_x_index)
                - d(self.node_v_index, self.node_y_index)
                + self.nodes[self.node_v].cumulated_reversal_distance
                + route_u.reversal_distance
                - self.nodes[self.node_x].cumulated_reversal_distance
                - route_u.penalty
                - route_v.penalty
        };

        // Early move pruning to save CPU time: this move cannot improve
        // without checking additional (load, duration...) constraints
        if cost >= 0.0 {
            return false;
        }

        {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            let node_u = &self.nodes[self.node_u];
            let node_v = &self.nodes[self.node_v];
            let node_x = &self.nodes[self.node_x];
            cost += self.penalty_excess_duration(
                p,
                node_u.cumulated_time
                    + node_v.cumulated_time
                    + node_v.cumulated_reversal_distance
                    + d(self.node_u_index, self.node_v_index),
            ) + self.penalty_excess_duration(
                p,
                route_u.duration - node_u.cumulated_time - d(self.node_u_index, self.node_x_index)
                    + route_u.reversal_distance
                    - node_x.cumulated_reversal_distance
                    + route_v.duration
                    - node_v.cumulated_time
                    - d(self.node_v_index, self.node_y_index)
                    + d(self.node_x_index, self.node_y_index),
            ) + self.penalty_excess_load(p, node_u.cumulated_load + node_v.cumulated_load)
                + self.penalty_excess_load(
                    p,
                    route_u.load + route_v.load - node_u.cumulated_load - node_v.cumulated_load,
                );
        }

        if cost > -MY_EPSILON {
            return false;
        }

        let depot_u = self.routes[self.route_u].depot;
        let depot_v = self.routes[self.route_v].depot;
        let depot_u_fin = self.nodes[depot_u].prev;
        let depot_v_fin = self.nodes[depot_v].prev;
        let depot_v_suiv = self.nodes[depot_v].next;

        // Reverse the tail of route U and append it to route V (and vice versa)
        let mut xx = self.node_x;
        while !self.nodes[xx].is_depot {
            let temp = self.nodes[xx].next;
            let node = &mut self.nodes[xx];
            std::mem::swap(&mut node.next, &mut node.prev);
            node.route = self.route_v;
            xx = temp;
        }

        let mut vv = self.node_v;
        while !self.nodes[vv].is_depot {
            let temp = self.nodes[vv].prev;
            let node = &mut self.nodes[vv];
            std::mem::swap(&mut node.prev, &mut node.next);
            node.route = self.route_u;
            vv = temp;
        }

        self.nodes[self.node_u].next = self.node_v;
        self.nodes[self.node_v].prev = self.node_u;
        self.nodes[self.node_x].next = self.node_y;
        self.nodes[self.node_y].prev = self.node_x;

        if self.nodes[self.node_x].is_depot {
            self.nodes[depot_u_fin].next = depot_u;
            self.nodes[depot_u_fin].prev = depot_v_suiv;
            self.nodes[depot_v_suiv].next = depot_u_fin;
            self.nodes[depot_v].next = self.node_y;
            self.nodes[self.node_y].prev = depot_v;
        } else if self.nodes[self.node_v].is_depot {
            self.nodes[depot_v].next = self.nodes[depot_u_fin].prev;
            let new_next = self.nodes[depot_v].next;
            self.nodes[new_next].prev = depot_v;
            self.nodes[depot_v].prev = depot_v_fin;
            self.nodes[depot_u_fin].prev = self.node_u;
            self.nodes[self.node_u].next = depot_u_fin;
        } else {
            self.nodes[depot_v].next = self.nodes[depot_u_fin].prev;
            let new_next = self.nodes[depot_v].next;
            self.nodes[new_next].prev = depot_v;
            self.nodes[depot_u_fin].prev = depot_v_suiv;
            self.nodes[depot_v_suiv].next = depot_u_fin;
        }

        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        self.update_route_data(p, self.route_v);
        true
    }

    /// If route(U) != route(V), replace (U,X) and (V,Y) by (U,Y) and (V,X).
    fn move9(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost = {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            d(self.node_u_index, self.node_y_index) + d(self.node_v_index, self.node_x_index)
                - d(self.node_u_index, self.node_x_index)
                - d(self.node_v_index, self.node_y_index)
                - route_u.penalty
                - route_v.penalty
        };

        // Early move pruning to save CPU time
        if cost >= 0.0 {
            return false;
        }

        {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            let node_u = &self.nodes[self.node_u];
            let node_v = &self.nodes[self.node_v];
            cost += self.penalty_excess_duration(
                p,
                node_u.cumulated_time + route_v.duration
                    - node_v.cumulated_time
                    - d(self.node_v_index, self.node_y_index)
                    + d(self.node_u_index, self.node_y_index),
            ) + self.penalty_excess_duration(
                p,
                route_u.duration - node_u.cumulated_time - d(self.node_u_index, self.node_x_index)
                    + node_v.cumulated_time
                    + d(self.node_v_index, self.node_x_index),
            ) + self.penalty_excess_load(
                p,
                node_u.cumulated_load + route_v.load - node_v.cumulated_load,
            ) + self.penalty_excess_load(
                p,
                node_v.cumulated_load + route_u.load - node_u.cumulated_load,
            );
        }

        if cost > -MY_EPSILON {
            return false;
        }

        let depot_u = self.routes[self.route_u].depot;
        let depot_v = self.routes[self.route_v].depot;
        let depot_u_fin = self.nodes[depot_u].prev;
        let depot_v_fin = self.nodes[depot_v].prev;
        let depot_u_pred = self.nodes[depot_u_fin].prev;

        // Swap the tails of the two routes
        let mut count = self.node_y;
        while !self.nodes[count].is_depot {
            self.nodes[count].route = self.route_u;
            count = self.nodes[count].next;
        }

        count = self.node_x;
        while !self.nodes[count].is_depot {
            self.nodes[count].route = self.route_v;
            count = self.nodes[count].next;
        }

        self.nodes[self.node_u].next = self.node_y;
        self.nodes[self.node_y].prev = self.node_u;
        self.nodes[self.node_v].next = self.node_x;
        self.nodes[self.node_x].prev = self.node_v;

        if self.nodes[self.node_x].is_depot {
            self.nodes[depot_u_fin].prev = self.nodes[depot_v_fin].prev;
            let new_prev = self.nodes[depot_u_fin].prev;
            self.nodes[new_prev].next = depot_u_fin;
            self.nodes[self.node_v].next = depot_v_fin;
            self.nodes[depot_v_fin].prev = self.node_v;
        } else {
            self.nodes[depot_u_fin].prev = self.nodes[depot_v_fin].prev;
            let new_prev = self.nodes[depot_u_fin].prev;
            self.nodes[new_prev].next = depot_u_fin;
            self.nodes[depot_v_fin].prev = depot_u_pred;
            self.nodes[depot_u_pred].next = depot_v_fin;
        }

        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        self.update_route_data(p, self.route_v);
        true
    }

    /// Calculates all SWAP* moves between route_u and route_v and applies the best improving one.
    fn swap_star(&mut self, p: &Params) -> bool {
        let mut best = SwapStarElement::default();

        // Preprocessing insertion costs
        self.preprocess_insertions(p, self.route_u, self.route_v);
        self.preprocess_insertions(p, self.route_v, self.route_u);

        let route_u = self.route_u;
        let route_v = self.route_v;
        let depot_u_next = self.nodes[self.routes[route_u].depot].next;
        let depot_v_next = self.nodes[self.routes[route_v].depot].next;
        let d = |i: usize, j: usize| p.time_cost.get(i, j);

        // Evaluating the moves
        let mut u = depot_u_next;
        while !self.nodes[u].is_depot {
            let u_cour = self.nodes[u].cour;
            let mut v = depot_v_next;
            while !self.nodes[v].is_depot {
                let v_cour = self.nodes[v].cour;
                let delta_pen_route_u = self.penalty_excess_load(
                    p,
                    self.routes[route_u].load + p.clients[v_cour].demand - p.clients[u_cour].demand,
                ) - self.routes[route_u].penalty;
                let delta_pen_route_v = self.penalty_excess_load(
                    p,
                    self.routes[route_v].load + p.clients[u_cour].demand - p.clients[v_cour].demand,
                ) - self.routes[route_v].penalty;

                // Quick filter: possibly early elimination of many SWAP* due to the
                // capacity constraints/penalties and bounds on insertion costs
                if delta_pen_route_u
                    + self.nodes[u].delta_removal
                    + delta_pen_route_v
                    + self.nodes[v].delta_removal
                    <= 0.0
                {
                    // Evaluate best reinsertion cost of U in the route of V where V has been removed
                    let (extra_v, best_position_u) =
                        self.get_cheapest_insert_simult_removal(p, u, v);
                    // Evaluate best reinsertion cost of V in the route of U where U has been removed
                    let (extra_u, best_position_v) =
                        self.get_cheapest_insert_simult_removal(p, v, u);

                    // Evaluating final cost
                    let move_cost = delta_pen_route_u
                        + self.nodes[u].delta_removal
                        + extra_u
                        + delta_pen_route_v
                        + self.nodes[v].delta_removal
                        + extra_v
                        + self.penalty_excess_duration(
                            p,
                            self.routes[route_u].duration
                                + self.nodes[u].delta_removal
                                + extra_u
                                + p.clients[v_cour].service_duration
                                - p.clients[u_cour].service_duration,
                        )
                        + self.penalty_excess_duration(
                            p,
                            self.routes[route_v].duration + self.nodes[v].delta_removal + extra_v
                                - p.clients[v_cour].service_duration
                                + p.clients[u_cour].service_duration,
                        );

                    if move_cost < best.move_cost {
                        best = SwapStarElement {
                            move_cost,
                            u: Some(u),
                            best_position_u,
                            v: Some(v),
                            best_position_v,
                        };
                    }
                }
                v = self.nodes[v].next;
            }
            u = self.nodes[u].next;
        }

        // Including RELOCATE from node_u towards route_v (costs nothing to include in the evaluation
        // at this step since we already have the best insertion location).
        // Moreover, since the granularity criterion is different, this can lead to different improving moves.
        let mut u = depot_u_next;
        while !self.nodes[u].is_depot {
            let u_cour = self.nodes[u].cour;
            let bi = &self.best_insert_client[self.bi_index(route_v, u_cour)];
            let best_position_u = bi.best_location[0];
            let delta_dist_route_v = bi.best_cost[0];
            let u_prev_cour = self.nodes[self.nodes[u].prev].cour;
            let u_next_cour = self.nodes[self.nodes[u].next].cour;
            let delta_dist_route_u =
                d(u_prev_cour, u_next_cour) - d(u_prev_cour, u_cour) - d(u_cour, u_next_cour);
            let move_cost = delta_dist_route_u
                + delta_dist_route_v
                + self.penalty_excess_load(p, self.routes[route_u].load - p.clients[u_cour].demand)
                - self.routes[route_u].penalty
                + self.penalty_excess_load(p, self.routes[route_v].load + p.clients[u_cour].demand)
                - self.routes[route_v].penalty
                + self.penalty_excess_duration(
                    p,
                    self.routes[route_u].duration + delta_dist_route_u
                        - p.clients[u_cour].service_duration,
                )
                + self.penalty_excess_duration(
                    p,
                    self.routes[route_v].duration
                        + delta_dist_route_v
                        + p.clients[u_cour].service_duration,
                );

            if move_cost < best.move_cost {
                best = SwapStarElement {
                    move_cost,
                    u: Some(u),
                    best_position_u,
                    v: None,
                    best_position_v: None,
                };
            }
            u = self.nodes[u].next;
        }

        // Including RELOCATE from node_v towards route_u
        let mut v = depot_v_next;
        while !self.nodes[v].is_depot {
            let v_cour = self.nodes[v].cour;
            let bi = &self.best_insert_client[self.bi_index(route_u, v_cour)];
            let best_position_v = bi.best_location[0];
            let delta_dist_route_u = bi.best_cost[0];
            let v_prev_cour = self.nodes[self.nodes[v].prev].cour;
            let v_next_cour = self.nodes[self.nodes[v].next].cour;
            let delta_dist_route_v =
                d(v_prev_cour, v_next_cour) - d(v_prev_cour, v_cour) - d(v_cour, v_next_cour);
            let move_cost = delta_dist_route_u
                + delta_dist_route_v
                + self.penalty_excess_load(p, self.routes[route_u].load + p.clients[v_cour].demand)
                - self.routes[route_u].penalty
                + self.penalty_excess_load(p, self.routes[route_v].load - p.clients[v_cour].demand)
                - self.routes[route_v].penalty
                + self.penalty_excess_duration(
                    p,
                    self.routes[route_u].duration
                        + delta_dist_route_u
                        + p.clients[v_cour].service_duration,
                )
                + self.penalty_excess_duration(
                    p,
                    self.routes[route_v].duration + delta_dist_route_v
                        - p.clients[v_cour].service_duration,
                );

            if move_cost < best.move_cost {
                best = SwapStarElement {
                    move_cost,
                    u: None,
                    best_position_u: None,
                    v: Some(v),
                    best_position_v,
                };
            }
            v = self.nodes[v].next;
        }

        if best.move_cost > -MY_EPSILON {
            return false;
        }

        // Applying the best move in case of improvement
        if let Some(position) = best.best_position_u {
            self.insert_node(
                best.u.expect("U is set together with its position"),
                position,
            );
        }
        if let Some(position) = best.best_position_v {
            self.insert_node(
                best.v.expect("V is set together with its position"),
                position,
            );
        }
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, route_u);
        self.update_route_data(p, route_v);
        true
    }

    /// Calculates the insertion cost and position of U in the route of V, where V is omitted.
    /// Returns (best_cost, best_position).
    fn get_cheapest_insert_simult_removal(
        &self,
        p: &Params,
        u: usize,
        v: usize,
    ) -> (f64, Option<usize>) {
        let best_insert =
            &self.best_insert_client[self.bi_index(self.nodes[v].route, self.nodes[u].cour)];

        // Find the best insertion in the route such that V is not next or pred
        // (the optimal insertion in that case can only belong to the top three locations)
        let mut best_position = best_insert.best_location[0];
        let mut best_cost = best_insert.best_cost[0];
        let mut found = match best_position {
            Some(position) => position != v && self.nodes[position].next != v,
            None => false,
        };
        if !found && best_insert.best_location[1].is_some() {
            best_position = best_insert.best_location[1];
            best_cost = best_insert.best_cost[1];
            let position = best_position.expect("checked above");
            found = position != v && self.nodes[position].next != v;
            if !found && best_insert.best_location[2].is_some() {
                best_position = best_insert.best_location[2];
                best_cost = best_insert.best_cost[2];
                found = true;
            }
        }

        // Also test the insertion in the place of V
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let v_prev = self.nodes[v].prev;
        let v_next = self.nodes[v].next;
        let delta_cost = d(self.nodes[v_prev].cour, self.nodes[u].cour)
            + d(self.nodes[u].cour, self.nodes[v_next].cour)
            - d(self.nodes[v_prev].cour, self.nodes[v_next].cour);
        if !found || delta_cost < best_cost {
            best_position = Some(v_prev);
            best_cost = delta_cost;
        }

        (best_cost, best_position)
    }

    /// Preprocesses all insertion costs of customers of route r1 in route r2.
    fn preprocess_insertions(&mut self, p: &Params, r1: usize, r2: usize) {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut u = self.nodes[self.routes[r1].depot].next;
        while !self.nodes[u].is_depot {
            // Compute the cost of removing U from its route
            let u_cour = self.nodes[u].cour;
            let u_prev_cour = self.nodes[self.nodes[u].prev].cour;
            let u_next = self.nodes[u].next;
            let u_next_cour = self.nodes[u_next].cour;
            self.nodes[u].delta_removal =
                d(u_prev_cour, u_next_cour) - d(u_prev_cour, u_cour) - d(u_cour, u_next_cour);

            // Recompute the table of best insertions in r2 only if it was modified since
            let bi_idx = self.bi_index(r2, u_cour);
            if self.routes[r2].when_last_modified
                > self.best_insert_client[bi_idx].when_last_calculated
            {
                self.best_insert_client[bi_idx].reset();
                self.best_insert_client[bi_idx].when_last_calculated = self.nb_moves;

                let depot2 = self.routes[r2].depot;
                let first = self.nodes[depot2].next;
                let first_cour = self.nodes[first].cour;
                self.best_insert_client[bi_idx].best_cost[0] =
                    d(0, u_cour) + d(u_cour, first_cour) - d(0, first_cour);
                self.best_insert_client[bi_idx].best_location[0] = Some(depot2);

                let mut v = first;
                while !self.nodes[v].is_depot {
                    let v_cour = self.nodes[v].cour;
                    let v_next = self.nodes[v].next;
                    let v_next_cour = self.nodes[v_next].cour;
                    let delta_cost =
                        d(v_cour, u_cour) + d(u_cour, v_next_cour) - d(v_cour, v_next_cour);
                    self.best_insert_client[bi_idx].compare_and_add(delta_cost, v);
                    v = v_next;
                }
            }
            u = u_next;
        }
    }

    /// Solution update: insert U after V.
    fn insert_node(&mut self, u: usize, v: usize) {
        let u_prev = self.nodes[u].prev;
        let u_next = self.nodes[u].next;
        self.nodes[u_prev].next = u_next;
        self.nodes[u_next].prev = u_prev;
        let v_next = self.nodes[v].next;
        self.nodes[v_next].prev = u;
        self.nodes[u].prev = v;
        self.nodes[u].next = v_next;
        self.nodes[v].next = u;
        self.nodes[u].route = self.nodes[v].route;
    }

    /// Solution update: swap U and V.
    fn swap_node(&mut self, u: usize, v: usize) {
        let v_prev = self.nodes[v].prev;
        let v_next = self.nodes[v].next;
        let u_prev = self.nodes[u].prev;
        let u_next = self.nodes[u].next;
        let route_u = self.nodes[u].route;
        let route_v = self.nodes[v].route;

        self.nodes[u_prev].next = v;
        self.nodes[u_next].prev = v;
        self.nodes[v_prev].next = u;
        self.nodes[v_next].prev = u;

        self.nodes[u].prev = v_prev;
        self.nodes[u].next = v_next;
        self.nodes[v].prev = u_prev;
        self.nodes[v].next = u_next;

        self.nodes[u].route = route_v;
        self.nodes[v].route = route_u;
    }

    /// Updates the preprocessed data of a route.
    fn update_route_data(&mut self, p: &Params, route: usize) {
        let mut my_place = 0;
        let mut my_load = 0.0;
        let mut my_time = 0.0;
        let mut my_reversal_distance = 0.0;
        let mut cumulated_x = 0.0;
        let mut cumulated_y = 0.0;

        let depot = self.routes[route].depot;
        {
            let node = &mut self.nodes[depot];
            node.position = 0;
            node.cumulated_load = 0.0;
            node.cumulated_time = 0.0;
            node.cumulated_reversal_distance = 0.0;
        }

        let mut my_node = depot;
        let mut first_it = true;
        while !self.nodes[my_node].is_depot || first_it {
            my_node = self.nodes[my_node].next;
            my_place += 1;
            let cour = self.nodes[my_node].cour;
            let prev_cour = self.nodes[self.nodes[my_node].prev].cour;
            my_load += p.clients[cour].demand;
            my_time += p.time_cost.get(prev_cour, cour) + p.clients[cour].service_duration;
            my_reversal_distance +=
                p.time_cost.get(cour, prev_cour) - p.time_cost.get(prev_cour, cour);
            {
                let node = &mut self.nodes[my_node];
                node.position = my_place;
                node.cumulated_load = my_load;
                node.cumulated_time = my_time;
                node.cumulated_reversal_distance = my_reversal_distance;
            }
            if !self.nodes[my_node].is_depot {
                cumulated_x += p.clients[cour].coord_x;
                cumulated_y += p.clients[cour].coord_y;
                if first_it {
                    self.routes[route]
                        .sector
                        .initialize(p.clients[cour].polar_angle);
                } else {
                    self.routes[route]
                        .sector
                        .extend(p.clients[cour].polar_angle);
                }
            }
            first_it = false;
        }

        let penalty =
            self.penalty_excess_duration(p, my_time) + self.penalty_excess_load(p, my_load);
        let nb_customers = my_place - 1;
        {
            let r = &mut self.routes[route];
            r.duration = my_time;
            r.load = my_load;
            r.penalty = penalty;
            r.nb_customers = nb_customers;
            r.reversal_distance = my_reversal_distance;
            // Remember "when" this route has been last modified
            // (will be used to filter unnecessary move evaluations)
            r.when_last_modified = self.nb_moves;
        }

        if nb_customers == 0 {
            self.routes[route].polar_angle_barycenter = 1.0e30;
            self.empty_routes.insert(route);
        } else {
            self.routes[route].polar_angle_barycenter = (cumulated_y / nb_customers as f64
                - p.clients[0].coord_y)
                .atan2(cumulated_x / nb_customers as f64 - p.clients[0].coord_x);
            self.empty_routes.remove(&route);
        }
    }

    /// Loads an initial solution into the local search structures.
    pub fn load_individual(&mut self, params: &Params, indiv: &Individual) {
        self.empty_routes.clear();
        self.nb_moves = 0;
        for r in 0..self.nb_vehicles {
            let my_depot = self.depot_node(r);
            let my_depot_fin = self.depot_end_node(r);
            self.nodes[my_depot].prev = my_depot_fin;
            self.nodes[my_depot_fin].next = my_depot;
            if !indiv.chrom_r[r].is_empty() {
                let mut my_client = indiv.chrom_r[r][0];
                self.nodes[my_client].route = r;
                self.nodes[my_client].prev = my_depot;
                self.nodes[my_depot].next = my_client;
                for i in 1..indiv.chrom_r[r].len() {
                    let my_client_pred = my_client;
                    my_client = indiv.chrom_r[r][i];
                    self.nodes[my_client].prev = my_client_pred;
                    self.nodes[my_client_pred].next = my_client;
                    self.nodes[my_client].route = r;
                }
                self.nodes[my_client].next = my_depot_fin;
                self.nodes[my_depot_fin].prev = my_client;
            } else {
                self.nodes[my_depot].next = my_depot_fin;
                self.nodes[my_depot_fin].prev = my_depot;
            }
            self.update_route_data(params, r);
            self.routes[r].when_last_tested_swap_star = -1;
            for i in 1..=self.nb_clients {
                // Initializing memory structures
                let bi_idx = self.bi_index(r, i);
                self.best_insert_client[bi_idx].when_last_calculated = -1;
            }
        }

        for i in 1..=self.nb_clients {
            // Initializing memory structures
            self.nodes[i].when_last_tested_ri = -1;
        }
    }

    /// Exports the LS solution into an individual and computes the penalized cost
    /// according to the original penalty weights from Params.
    pub fn export_individual(&self, params: &Params, indiv: &mut Individual) {
        // Empty routes have a polar angle of 1.e30 and therefore always appear at the end
        let mut route_polar_angles: Vec<(f64, usize)> = (0..self.nb_vehicles)
            .map(|r| (self.routes[r].polar_angle_barycenter, r))
            .collect();
        route_polar_angles.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));

        let mut pos = 0;
        for r in 0..self.nb_vehicles {
            indiv.chrom_r[r].clear();
            let mut node = self.nodes[self.routes[route_polar_angles[r].1].depot].next;
            while !self.nodes[node].is_depot {
                indiv.chrom_t[pos] = self.nodes[node].cour;
                indiv.chrom_r[r].push(self.nodes[node].cour);
                node = self.nodes[node].next;
                pos += 1;
            }
        }

        indiv.evaluate_complete_cost(params);
    }
}

```

</details>
<details>
<summary>Tool Call: rust/src/lib.rs</summary>

*Status: Completed*

Diff: rust/src/lib.rs
```
//! HGS-CVRP: A Rust implementation of the Hybrid Genetic Search for the
//! Capacitated Vehicle Routing Problem, with the SWAP* neighborhood.
//!
//! This is a faithful port of the C++ reference implementation by Thibaut Vidal
//! (<https://github.com/vidalt/HGS-CVRP>, MIT license), described in:
//!
//! - Vidal, T., Crainic, T. G., Gendreau, M., Lahrichi, N., Rei, W. (2012).
//!   A hybrid genetic algorithm for multidepot and periodic vehicle routing problems.
//!   Operations Research, 60(3), 611-624.
//! - Vidal, T. (2022). Hybrid genetic search for the CVRP: Open-source implementation
//!   and SWAP* neighborhood. Computers & Operations Research, 140, 105643.
//!
//! # Example
//!
//! ```no_run
//! use hgs_cvrp::{AlgorithmParameters, CvrplibInstance, Genetic, Params};
//!
//! let instance = CvrplibInstance::read("instance.vrp", true).unwrap();
//! let params = Params::new(
//!     &instance.x_coords, &instance.y_coords, instance.dist_mtx,
//!     &instance.service_time, &instance.demands,
//!     instance.vehicle_capacity, instance.duration_limit,
//!     None, instance.is_duration_constraint, true,
//!     AlgorithmParameters::default(),
//! ).unwrap();
//! let mut solver = Genetic::new(params);
//! solver.run();
//! if let Some(best) = solver.population.best_found() {
//!     println!("best cost: {}", best.eval.penalized_cost);
//! }
//! ```

// These stylistic lints are deliberately not followed: the code intentionally
// mirrors the structure of the C++ reference implementation for traceability.
#![allow(clippy::manual_is_multiple_of)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::manual_rem_euclid)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::should_implement_trait)]

pub mod algorithm_parameters;
pub mod circle_sector;
pub mod cli;
pub mod cvrplib;
pub mod genetic;
pub mod individual;
pub mod local_search;
pub mod matrix;
pub mod params;
pub mod population;
pub mod rng;
pub mod split;
pub mod util;

pub use algorithm_parameters::AlgorithmParameters;
pub use cli::CommandLine;
pub use cvrplib::CvrplibInstance;
pub use genetic::Genetic;
pub use individual::{export_cvrplib_format, EvalIndiv, Individual};
pub use local_search::LocalSearch;
pub use matrix::SquareMatrix;
pub use params::{Client, Params};
pub use population::Population;
pub use split::Split;
pub use util::format_double;

```

</details>
<details>
<summary>Tool Call: cd rust && cargo clippy --release 2>&1 | grep -cE "^warning: " ; cargo test --release 2>&1 | grep -E "test result"</summary>

*Status: Completed*

Terminal:
```
0
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


```

</details>
## Assistant

<details>
<summary>思考过程</summary>

Clippy's clean and tests are passing, so I'm running through one of the official tests again to make sure the mem::swap refactor didn't change the behavior — the swap should be semantically equivalent, but I want to verify the results match.Clippy passed with zero warnings and tests are passing. I'll re-verify the official tests to ensure the `mem::swap` changes don't alter behavior on the core move7/move8 paths.

</details>

<details>
<summary>Tool Call: cd rust && cargo build --release 2>&1 | tail -1 && ./target/release/hgs ../Instances/CVRP/X-n101-k25.vrp /tmp/rust_X101v2.sol -seed 1 -log 0 && tail -1 /tmp/rust_X101v2.sol && ./target/release/hgs ../Instances/CVRP/CMT6.vrp /tmp/rust_CMT6v2.sol -seed 1 -round 0 -log 0 && tail -1 /tmp/rust_CMT6v2.sol</summary>

*Status: Completed*

Terminal:
```
    Finished `release` profile [optimized] target(s) in 3.24s
Cost 27591
Cost 555.43



```

</details>
## Assistant

<details>
<summary>思考过程</summary>

The behavior is preserved since the swap operation is semantically identical, and with the same seed the output is deterministic, confirming no changes. Now I'm comparing the two solutions against the previous run to verify they're exactly the same.

</details>

与改动前完全一致(同 seed 确定性输出)。最后对比确认解文件逐字节一致,然后写 README 和 `.gitignore`:

<details>
<summary>Tool Call: diff /tmp/rust_X101.sol /tmp/rust_X101v2.sol && echo "X101 IDENTICAL"; diff /tmp/rust_CMT6.sol /tmp/rust_CMT6v2.sol && echo "CMT6 IDENTICAL"</summary>

*Status: Completed*

Terminal:
```
X101 IDENTICAL
CMT6 IDENTICAL




```

</details>
<details>
<summary>Tool Call: rust/README.md</summary>

*Status: Completed*

Diff: rust/README.md
```
# HGS-CVRP in Rust

A Rust port of the [HGS-CVRP](https://github.com/vidalt/HGS-CVRP) reference implementation:
the Hybrid Genetic Search for the Capacitated Vehicle Routing Problem with the SWAP*
neighborhood (Vidal 2022, MIT license).

The port is faithful: every algorithmic component (Split, granular RI local search,
SWAP*, population/diversity management, adaptive penalties) follows the C++ code
line by line, so results and performance are directly comparable.

## Building and running

```console
cargo build --release
./target/release/hgs ../Instances/CVRP/X-n157-k13.vrp mySolution.sol -seed 1 -t 30
```

The command line interface is identical to the C++ executable (same flags, same
defaults, same log and solution file formats). Run without arguments to print the help.

Run the unit tests with `cargo test`.

## Library usage

The crate also exposes the solver as a library (see the example in `src/lib.rs`):
read or build an instance, construct `Params`, then run `Genetic`. This plays the
role of the C interface in the original project.

## Code structure

The modules map one-to-one to the C++ source files:

| Rust module               | C++ file              | Content                                            |
|---------------------------|-----------------------|----------------------------------------------------|
| `params`                  | `Params.*`            | Instance data, penalties, RNG, correlated vertices |
| `individual`              | `Individual.*`        | Solution representation, evaluation, solution I/O  |
| `population`              | `Population.*`        | Subpopulations, diversity, penalties, best tracking|
| `genetic`                 | `Genetic.*`           | Main GA loop and OX crossover                      |
| `local_search`            | `LocalSearch.*`       | RI moves (1-9) and SWAP* neighborhood              |
| `split`                   | `Split.*`             | Linear Split (limited and unlimited fleet)         |
| `circle_sector`           | `CircleSector.h`      | Circle sectors for SWAP* pruning                   |
| `algorithm_parameters`    | `AlgorithmParameters.*` | HGS parameters and defaults                      |
| `cvrplib`                 | `InstanceCVRPLIB.*`   | CVRPLIB instance reader                            |
| `cli`                     | `commandline.h`       | Command line parsing                               |
| `rng`, `matrix`, `util`   | -                     | minstd LCG, flat distance matrix, %g formatting    |

Design notes (differences imposed or encouraged by Rust):

- **No pointers, no reference counting.** The linked list of the local search lives in
  a single `Vec<Node>` arena and links are indices; population members are identified
  by ids instead of addresses. Updates remain O(1) and there is no `Rc`/`RefCell`/unsafe.
- **Mutable state is threaded explicitly.** The C++ classes all hold `Params&` and
  mutate the penalties/RNG through it; here a `&mut Params` is passed to the calls
  that need it, which makes the data flow visible and keeps the borrow checker happy.
- **The only solution copies are the semantically required ones** (storing an
  individual into the population, tracking the best solution; the latter uses
  `clone_from` to recycle buffers).

## Behavioral parity with the C++ implementation

Verified on the same test set as the upstream CI (seed 1, default termination):

| Instance     | Options    | C++ cost | Rust cost |
|--------------|------------|----------|-----------|
| X-n101-k25   | `-round 1` | 27591    | 27591     |
| X-n110-k13   | `-round 1` | 14971    | 14971     |
| CMT6         | `-round 0` | 555.43   | 555.43    |
| CMT7         | `-round 0` | 909.675  | 909.675   |

Time per iteration is on par with the C++ build (within a few percent on X-n502-k39).

Known, intentional differences:

- **Random streams.** The RNG engine is the same `minstd_rand` LCG, but `shuffle` and
  the uniform distributions are own implementations (the C++ standard leaves them
  implementation-defined), so the sequence of visited solutions differs from a given
  libstdc++ build. Solution quality is statistically equivalent.
- **Clock.** Times are measured with a monotonic wall clock instead of `clock()`
  (CPU time); both coincide for this single-threaded program.
- **Errors.** User-facing errors (bad command line, unreadable instance) are reported
  as `EXCEPTION | ...` like the C++ version; internal invariant violations
  (e.g. a failed Split propagation) panic instead of throwing.
- Invalid numeric command-line values are rejected with an error message
  (C++ `atoi` would silently read 0).

```

</details>
<details>
<summary>Tool Call: rust/.gitignore</summary>

*Status: Completed*

Diff: rust/.gitignore
```
/target
Cargo.lock
*.sol
*.PG.csv

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

I should reconsider the Cargo.lock file — even though this project has no dependencies, the modern Rust convention is to commit it for both libraries and binaries, so I'll remove it from .gitignore. I was about to run something but got cut off.

Let me do a final check of the project and clean up that C++ build directory I created for testing, then verify what the root .gitignore already covers.我来修正 `.gitignore` 确保 `Cargo.lock` 被提交，并检查根目录的 `.gitignore` 是否正确忽略 `build/` 目录。

</details>

<details>
<summary>Tool Call: rust/.gitignore</summary>

*Status: Completed*

Diff: rust/.gitignore
```
/target
*.sol
*.PG.csv

```

</details>
<details>
<summary>Tool Call: Read file `.gitignore`</summary>

*Status: Completed*

```HGS-CVRP/.gitignore
     1	# Build directory
     2	build
     3	
     4	# Test build files
     5	Test/CMakeFiles/
     6	Test/CMakeCache.txt
     7	
     8	# IDE files
     9	.vscode
    10	.idea
    11	cmake-build-*
    12	[Dd]ebug/
    13	[Dd]ebugPublic/
    14	[Rr]elease/
    15	[Rr]eleases/
    16	[Xx]64/
    17	[Xx]86/
    18	*.user
    19	.vs/
    20	
    21	
    22	# Prerequisites
    23	*.d
    24	
    25	# Compiled Object files
    26	*.slo
    27	*.lo
    28	*.o
    29	*.obj
    30	
    31	# Precompiled Headers
    32	*.gch
    33	*.pch
    34	
    35	# Compiled Dynamic libraries
    36	*.so
    37	*.dylib
    38	*.dll
    39	
    40	# Fortran module files
    41	*.mod
    42	*.smod
    43	
    44	# Compiled Static libraries
    45	*.lai
    46	*.la
    47	*.a
    48	*.lib
    49	
    50	# Executables
    51	*.exe
    52	*.out
    53	*.app
    54	

```

</details>
## Assistant

<details>
<summary>思考过程</summary>

The root `.gitignore` is already handling the `build` directory, so that's covered. Now I'm running a final diagnostic check and reviewing what files I've created with `git status`.

</details>

根 `.gitignore` 已覆盖 `build/`。最后做一次完整检查 — 项目诊断 + git 状态:

<details>
<summary>Tool Call: Check project diagnostics</summary>

*Status: Completed*

Diagnostics successfully refreshed.

/home/username/HGS-CVRP/Program/main.cpp: 0 error(s), 2 warning(s)

</details>
<details>
<summary>Tool Call: git status --short | head -25</summary>

*Status: Completed*

Terminal:
```
?? rust/





```

</details>
## Assistant

<details>
<summary>思考过程</summary>

The Rust directory is clean with no issues, and I've already verified everything through testing and reference instances. Let me do a final build check with the locked code and review the directory structure to wrap this up.诊断确认只剩两个原有的 C++ 警告，与本次工作无关。

</details>

<details>
<summary>Tool Call: find rust -name target -prune -o -type f -print | sort; echo; wc -l rust/src/*.rs | tail -1</summary>

*Status: Completed*

Terminal:
```
rust/.gitignore
rust/Cargo.lock
rust/Cargo.toml
rust/README.md
rust/src/algorithm_parameters.rs
rust/src/circle_sector.rs
rust/src/cli.rs
rust/src/cvrplib.rs
rust/src/genetic.rs
rust/src/individual.rs
rust/src/lib.rs
rust/src/local_search.rs
rust/src/main.rs
rust/src/matrix.rs
rust/src/params.rs
rust/src/population.rs
rust/src/rng.rs
rust/src/split.rs
rust/src/util.rs

  3768 total

```

</details>
## Assistant

完成。HGS-CVRP 已在 `rust/` 目录下完整重新实现(约 3800 行,零第三方依赖)。

## 实现概览

**模块结构**与 C++ 源文件一一对应(`params`、`individual`、`population`、`genetic`、`local_search`、`split`、`circle_sector`、`cvrplib`、`cli`),另加三个小工具模块(`rng`、`matrix`、`util`),详见 `rust/README.md` 中的映射表。

**针对你的四点要求:**

1. **模块化与适度抽象** — 保持了原项目"简单、独立、专用"的理念:lib + bin 双目标(库 API 替代了原 C 接口的角色),没有引入 trait 层或泛型框架,模块边界即 C++ 类边界。
2. **注释** — 英文注释,保留了原代码中有价值的算法说明(如 SWAP* 剪枝、Split 支配规则),不为注释而注释。
3. **零成本抽象** — 无 `Rc`/`RefCell`/`unsafe`/第三方依赖:
   - LS 的双向链表用单一 `Vec<Node>` arena + 索引代替裸指针,更新仍为 O(1);
   - 种群的 `multiset<pair<double, Individual*>>` 用 `(distance, id)` 有序向量代替(且比指针地址 tie-break 更确定);
   - 距离矩阵为扁平 row-major `Vec<f64>`;
   - 仅有的 clone 是语义必需的(入种群存副本),最优解跟踪用 `clone_from` 复用缓冲;
   - C++ 中各类持有 `Params&` 并通过它改惩罚系数/RNG,Rust 中改为显式传 `&mut Params`,数据流更清晰。
4. **行为一致性** — 自实现了与 `std::minstd_rand` 同参数的 LCG(单测验证了标准规定的第 10000 个值),CLI/日志/解文件/PG.csv 格式逐字符对齐(包括模拟 C++ `%g` 浮点格式)。

## 验证结果

与本地编译的 C++ 参考版逐项对比(seed 1,默认 2 万次无改进迭代,即上游 CI 同款测试):

| 实例 | C++ | Rust |
|---|---|---|
| X-n101-k25 | 27591 | **27591** ✓ |
| X-n110-k13 | 14971 | **14971** ✓ |
| CMT6 (时长约束, round 0) | 555.43 | **555.43** ✓ |
| CMT7 (时长约束, round 0) | 909.675 | **909.675** ✓ |

性能:X-n502-k39 上单次迭代 3.89 ms (Rust) vs 4.02 ms (C++)。时间限制重启、指定车队、错误处理等边界路径也已验证一致。`cargo test`(7 个测试)与 `cargo clippy` 全部干净。

已知差异(README 中有记录):随机流不逐次相同(C++ 标准未规定 shuffle/分布的实现)、计时用挂钟而非 `clock()`、非法 CLI 数值会报错而非 `atoi` 静默归零。

要不要我再补充一个 GitHub Actions workflow(对应现有 `CI_Build.yml`)来自动跑这些 Rust 测试?


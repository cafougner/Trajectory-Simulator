import numpy as np
import os
import pandas as pd
from sklearn.linear_model import LinearRegression
from sklearn.preprocessing import PolynomialFeatures

def from_scientific(number):
    return f"{number:.128f}".rstrip("0").rstrip(".")

# These are both formatted for Desmos.
x0 = "d_i_s_t_a_n_c_e"
x1 = "v_e_l_o_c_i_t_y"

# These need to be changed in the results_analyzer.py and the constants.rs as well.
polynomialDegree = 8

resultsFolder = "results"
resultsFile = "results.csv"
equationFile = "equation.txt"

csvData = pd.read_csv(os.path.join(resultsFolder, resultsFile))

distances = csvData["distance"]
velocities = csvData["velocity"]
roots = csvData["root"]

polynomialGrid = np.vstack([distances, velocities]).T
polynomialFeatures = PolynomialFeatures(degree = polynomialDegree)
polynomial = polynomialFeatures.fit_transform(polynomialGrid)
model = LinearRegression(n_jobs = -1).fit(polynomial, roots)

polynomialPredictions = model.predict(polynomial)
errors = polynomialPredictions - roots

polynomialCoeffs = [from_scientific(coef) for coef in model.coef_]
polynomialFeatures = [name.replace("x0", x0).replace("x1", x1) for name in polynomialFeatures.get_feature_names_out()]
polynomialIntercept = from_scientific(model.intercept_)

maxError = np.max(np.abs(errors))
meanError = np.mean(np.abs(errors))

polynomialEquation = " + ".join(f"{coef}*{feature}" for coef, feature in zip(polynomialCoeffs, polynomialFeatures))

with open(os.path.join(resultsFolder, equationFile), "w", encoding = "utf-8") as f:
    f.write(f"Polynomial degree {polynomialDegree}\n\nErrors (°):\nMax: {maxError}\nMean: {meanError}\n\ny = {polynomialIntercept} + {polynomialEquation}")

print(f"Equation generated and written to {equationFile} in the {resultsFolder} folder.")

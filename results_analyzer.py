import matplotlib.pyplot as plt
import numpy as np
import os
import pandas as pd
from scipy.interpolate import griddata
from sklearn.linear_model import LinearRegression
from sklearn.preprocessing import PolynomialFeatures

# By default, this is overridden and all of the CSV data is used.
gridResolution = 500j

# These need to be changed in the results_analyzer.py and the constants.rs as well.
polynomialDegree = 8

resultsFolder = "results"
resultsFile = "results.csv"

csvData = pd.read_csv(os.path.join(resultsFolder, resultsFile))

distances = csvData["distance"]
velocities = csvData["velocity"]
roots = csvData["root"]

polynomialGrid = np.vstack([distances, velocities]).T
polynomialFeatures = PolynomialFeatures(degree = polynomialDegree)
polynomial = polynomialFeatures.fit_transform(polynomialGrid)
model = LinearRegression(n_jobs = -1).fit(polynomial, roots)

polynomialRoots = model.predict(polynomial)
polynomialErrors = polynomialRoots - roots

# Uncomment to use all of the CSV data.
gridResolution = complex(roots.__len__())

plot = plt.figure()
subplot1 = plot.add_subplot(131, projection = "3d")
subplot2 = plot.add_subplot(132, projection = "3d")
subplot3 = plot.add_subplot(133, projection = "3d")

distancesGrid, velocitiesGrid = np.mgrid[
    distances.min(): distances.max(): gridResolution,
    velocities.min(): velocities.max() : gridResolution
]

csvRootGrid = griddata((distances, velocities), roots, (distancesGrid, velocitiesGrid))
csvRootPlane = subplot1.plot_surface(distancesGrid, velocitiesGrid, csvRootGrid, cmap = "coolwarm", alpha = 1.0)

subplot1.set_title("Simulation Data")
subplot1.set_xlabel("Distance (m)")
subplot1.set_ylabel("Velocity (m/s)")
subplot1.set_zlabel("Angle (°)")

subplot1.view_init(elev = 25, azim = 60)

polynomialRootGrid = griddata((distances, velocities), polynomialRoots, (distancesGrid, velocitiesGrid))
polynomialRootPlane = subplot2.plot_surface(distancesGrid, velocitiesGrid, polynomialRootGrid, cmap = "coolwarm", alpha = 1.0)

subplot2.set_title("Polynomial Predictions")
subplot2.set_xlabel("Distance (m)")
subplot2.set_ylabel("Velocity (m/s)")
subplot2.set_zlabel("Angle (°)")

subplot2.view_init(elev = 25, azim = 60)

polynomialErrorGrid = griddata((distances, velocities), polynomialErrors, (distancesGrid, velocitiesGrid))
polynomialErrorPlane = subplot3.plot_surface(distancesGrid, velocitiesGrid, polynomialErrorGrid, cmap = "magma", alpha = 1.0)

subplot3.set_title("Polynomial Error")
subplot3.set_xlabel("Distance (m)")
subplot3.set_ylabel("Velocity (m/s)")
subplot3.set_zlabel("Error (°)")

subplot3.view_init(elev = 0, azim = 45)

# See https://stackoverflow.com/a/19823837
plt.get_current_fig_manager().window.state("zoomed")

plt.tight_layout()
plt.show()

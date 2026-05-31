import numpy
import matplotlib
import matplotlib.pyplot as plt
import pandas
from lsst.rsp import get_tap_service, retrieve_query
import lsst.daf.butler as dafButler
import lsst.geom
import lsst.afw.display as afwDisplay

from lsst.rsp import get_query_history
hist=await get_query_history()
hist

my_adql_query = { "SELECT TOP 10000
    objectId,
    coord_ra,
    coord_dec,

    g_cModelFlux,
    r_cModelFlux,
    i_cModelFlux,

    g_kronRad,
    r_kronRad,
    i_kronRad,

    shape_xx,
    shape_xy,
    shape_yy,

    refExtendedness

FROM dp02_dc2_catalogs.Object
WHERE detect_isPrimary = 1" } 
service = get_tap_service("tap")
results = service.search(my_adql_query)
results_table = results.to_table()

print(results_table)


my_adql_query = """
SELECT column_name, datatype, description
FROM TAP_SCHEMA.columns
WHERE table_name = 'dp02_dc2_catalogs.Object'
ORDER BY column_name
"""

service = get_tap_service("tap")
results = service.search(my_adql_query)
results_table = results.to_table()

import os

print(os.getcwd())

results_table.write(
    "dc2_objects.txt",
    format="ascii.tab",
    overwrite=True
)

print(results_table)

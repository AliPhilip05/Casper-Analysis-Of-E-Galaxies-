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

my_adql_query = { "SELECT TOP 100000 coord_ra, coord_dec FROM dp02_dc2_catalogs.Object" } 
service = get_tap_service("tap")
results = service.search(my_adql_query)
results_table = results.to_table()

print(results_table)

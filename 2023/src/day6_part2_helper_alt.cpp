#define PY_SSIZE_T_CLEAN
#include <Python.h>

#include <cstdint>

#include "day6_part2_helper.hpp"

static PyObject *calculate_ways_extension(PyObject *self, PyObject *args) {
    std::uint64_t t_max, distance;

    // We don't need to do bounds checking here because we know for a fact that the integer
    // passed in here won't be out of bounds.
    if (!PyArg_ParseTuple(args, "KK", &t_max, &distance)) {
        return NULL;
    }

    return Py_BuildValue("i", calculate_ways_impl(t_max, distance));
}

static PyMethodDef HelperMethods[] = {
    {"calculate_ways", calculate_ways_extension, METH_VARARGS,
     "Calculate the number of ways as defined in Day 6 Part 2, using a C++ extension."},
    {NULL, NULL, 0, NULL} /* Sentinel */
};

static struct PyModuleDef helpermodule = {PyModuleDef_HEAD_INIT, "helper", /* name of module */
                                          NULL,                            /* module documentation, may be NULL */
                                          -1, /* size of per-interpreter state of the module,
                                                 or -1 if the module keeps state in global variables. */
                                          HelperMethods};

PyMODINIT_FUNC PyInit_helper() { return PyModule_Create(&helpermodule); }

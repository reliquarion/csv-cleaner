# csv-cleaner
This program reads through a CSV file when given data types and removes all lines that either cannot be cast to the given types or have the incorrect number of columns due to a shortcoming where the Pandas read_csv() function cannot do so automatically.

## Usage
The executable accepts command line arguments and rewrites the input data file to a cleaned version. Command line arguments come in the following syntax: `[executable] [datafile path] *[datatypes, space-seperated]`. This can be used inside Python programs through os.system(), subprocess.run(), etc. If passing in dtypes from an existing dataframe, they can be formatted through `' '.join([dtype.name() for dtype in df.dtypes.values()])`.
For inputting datatypes manually see [here](https://pandas.pydata.org/pandas-docs/stable/user_guide/basics.html#basics-dtypes) for their string representations.

## File Formatting
The CSV file inputted must have an unlabelled index column or the first column will end up being dropped.

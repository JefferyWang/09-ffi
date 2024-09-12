from algo import Matrix

data = [[1.0, 1.0], [2.0, 2.0]]
a = Matrix(data)
data1 = [[2.0, 2.0], [1.0, 1.0]]
b = Matrix(data1)
print(a.mul(b))

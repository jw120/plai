button_list = []

for i in range(5):
    button_list.append(lambda i=i: print(i))

for i in range(5):
    button_list.append(lambda: print(i))


for button in button_list:
    button()

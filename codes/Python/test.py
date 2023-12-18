class ListNode:
    def __init__(self, val: int):
        self.val: int = val
        self.next: ListNode | None =None
    
    def insert_list(self, insert_list: ListNode):
        insert_list.next = self.next
        self.next = insert_list
        
        

n0 = ListNode(1)
n1 = ListNode(2)
n2 = ListNode(3)
n3 = ListNode(5)
n4 = ListNode(4)

n0.next = n1
n1.next = n2
n2.next = n3
n3.next = n4

n5 = ListNode(7)

n4.insert_list(n5)

print (n4.val, n4.next)

from math import sqrt
def user_input_int(message):
    while True:
        try:
            val = int(input(message))
            return val
        except:
            print("숫자로 입력해주세용")

def solve(bias,user, point):
    i = bias-user
    if i < 0:
        raise ValueError("최애의 키가 님의 키보다 작아요") # copilot 에러메시지 똑똑하당
    return round(2 * sqrt(2*point*i - i*i))
    

bias_height = user_input_int("최애의 키를 적어보세용 뻥튀기 금지~")
user_height = user_input_int("님의 키를 적어보세용 ")
point_height = user_input_int("님 최애의 포인트 거리를 적어보세용 ")

try:
    ans = solve(bias_height, user_height, point_height)
    print(ans)
    print("최애와 사진을 찍기 위해서 최애는 다리를 무려 이만큼 벌려야하네여...웨딩사진 찍을 때 참고!")
except Exception as e:
    print(e)

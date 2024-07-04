import psutil
import socket
import time

ip = input()

def get_size(bytes, suffix="B"):
    """
    Scale bytes to its proper format
    e.g:
        1253656 => '1.20MB'
        1253656678 => '1.17GB'
    """
    factor = 1024
    for unit in ["", "K", "M", "G", "T", "P"]:
        if bytes < factor:
            return f"{bytes:.2f}{unit}{suffix}"
        bytes /= factor

def send(message1, message2, ip):
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    blank1 = "0§0                "
    blank2 = "0§1                "
    port = 80
    try:
        sock.sendto(blank1.encode(), (ip, port))
        sock.sendto(blank2.encode(), (ip, port))
        sock.sendto(message1.encode(), (ip, port))
        sock.sendto(message2.encode(), (ip, port))
    finally:
        sock.close()

while True:
    cpufreq = psutil.cpu_freq()
    svmem = psutil.virtual_memory()
    cpu = str(psutil.cpu_percent(1))+"%"
    freq = f"{cpufreq.current:.2f}Mhz"
    ram = get_size(svmem.used) +"/" + get_size(svmem.total)
    message1 = "0§0" + str(cpu) + " " + str(freq)
    message2 = "0§1" + ram
    send(message1, message2, ip)
    time.sleep(1)
using System.Text;

string[] lines = File.ReadAllLines("./../input.txt", Encoding.UTF8);

var oxygen = lines.ToList();
var carbon = lines.ToList();

for (int i = 0; i < lines[0].Length; i++)
{
    char digit = (oxygen.Count(l => l[i] == '1') * 2 / oxygen.Count) > 0 ? '1' : '0';
    oxygen = oxygen.Where(l => l[i] == digit).ToList();
    if (oxygen.Count == 1) break;
}

for (int i = 0; i < lines[0].Length; i++)
{
    char digit = (carbon.Count(l => l[i] == '1') * 2 / carbon.Count) > 0 ? '1' : '0';
    carbon = carbon.Where(l => l[i] != digit).ToList();
    if (carbon.Count == 1) break;
}

var O2 = Convert.ToInt32(oxygen[0], 2);
var CO2 = Convert.ToInt32(carbon[0], 2);
Console.WriteLine($"{oxygen[0]} --> {O2}");
Console.WriteLine($"{carbon[0]} --> {CO2}");
Console.WriteLine($"{O2} * {CO2} = {O2 * CO2} ");

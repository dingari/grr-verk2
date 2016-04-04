clear all; close all; clc;

[n, optimal1, greedy1, equal1, random1] = get_data('../data/test1.csv');
[n, optimal2, greedy2, equal2, random2] = get_data('../data/test2.csv');
[n, optimal3, greedy3, equal3, random3] = get_data('../data/test3.csv');
[n, optimal4, greedy4, equal4, random4] = get_data('../data/test4.csv');
[n, optimal5, greedy5, equal5, random5] = get_data('../data/test5.csv');

optimal = [optimal1 optimal2 optimal3 optimal4 optimal5];
greedy = [greedy1 greedy2 greedy3 greedy4 greedy5];
equal = [equal1 equal2 equal3 equal4 equal5];
random = [random1 random2 random3 random4 random5];

optimal_avg = (optimal1 + optimal2 + optimal3 + optimal4 + optimal5) / 5;
greedy_avg = (greedy1 + greedy2 + greedy3 + greedy4 + greedy5) / 5;
equal_avg = (equal1 + equal2 + equal3 + equal4 + equal5) / 5;
random_avg = (random1 + random2 + random3 + random4 + random5) / 5;

figure(1);
plot(n, optimal_avg, 'b'); hold on;
plot(n, greedy_avg, 'm');
plot(n, equal_avg, 'k');
plot(n, random_avg, 'r');

for i=1:5
    plot(n, optimal(:,i), 'bo');
    plot(n, greedy(:,i), 'mo');
    plot(n, equal(:,i), 'ko');
    plot(n, random(:,i), 'ro');
end

xlabel('n');
ylabel('Weighted path length');
legend('Optimal', 'Greedy', 'Equal', 'Random');

print -dpng 'images/fig1';

    
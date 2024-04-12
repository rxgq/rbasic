import pandas as pd
import random as r
from sklearn.svm import SVC
from sklearn import svm
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler
from sklearn.metrics import classification_report
from ucimlrepo import fetch_ucirepo 

def main():  
    breast_cancer_wisconsin_diagnostic = fetch_ucirepo(id=17) 
    
    X = breast_cancer_wisconsin_diagnostic.data.features 
    y = breast_cancer_wisconsin_diagnostic.data.targets 

    report = predict_diagnosis(X, y, showPredictions=True)
    print(report)


def predict_diagnosis(X, y, showPredictions):
    X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=r.randint(1, 100)) # trains on 80% of the data, tests 20%
    
    sc = StandardScaler()
    X_train = sc.fit_transform(X_train)
    X_test = sc.transform(X_test)

    clf = svm.SVC()
    clf.fit(X_train, y_train)
    pred_clf = clf.predict(X_test)

    report = classification_report(y_test, pred_clf)
    
    if (showPredictions is False): 
        return report
        
    for i in range(len(X_test)):
        print(f"Sample {i + 1}: P: {pred_clf[i]}, A: {y_test.iloc[i]}")

    return report

def init_data(data):
    data = pd.read_csv(data)

    data = data.drop(columns=['UID'])
    data['Diagnosis'] = data['Diagnosis'].map({'M': 1, 'B': 0})

    return data


if __name__ == "__main__": main()
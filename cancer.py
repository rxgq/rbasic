import pandas as pd
import sklearn
from sklearn.svm import SVC
from sklearn import svm
from sklearn.model_selection import train_test_split
from sklearn.preprocessing import StandardScaler

DATA_PATH = "data\data.csv"

def main():
    data = init_data(DATA_PATH)

    X = data.drop(columns=['Diagnosis'])
    y = data['Diagnosis']

    score = predict_diagnosis(X, y)

    print(score)


def predict_diagnosis(X, y):
    X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=7)
    
    sc = StandardScaler()
    X_train = sc.fit_transform(X_train)
    X_test = sc.transform(X_test)

    clf = svm.SVC()
    clf.fit(X_train, y_train)
    pred_clf = clf.predict(X_test)

    return sklearn.metrics.accuracy_score(y_test, pred_clf)


def init_data(data):
    data = pd.read_csv(data)

    data = data.drop(columns=['UID'])
    data['Diagnosis'] = data['Diagnosis'].map({'M': 1, 'B': 0})

    return data


if __name__ == "__main__": main()
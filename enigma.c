#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#define NQUAD 400000
#define N_MAX_STRINGA_CRITT  128

char alphabet[26];
char englishQuad[NQUAD][5];
long int englishQuadProb[NQUAD], numMaxQuad=0;

//funzione scambio
void scambio(int *a, int *b)
{
    int t;
    t=*a;
    *a=*b;
    *b=t;
}

//funzione sort per ordinare un vettore
int sort(int vet[])
{
    int qq,pp,vetOrdinato=0;
    for(qq=0;qq<3;qq++)
    {
        vetOrdinato=0;
        for(pp=0;pp<3-qq-1;pp++)
        {
            if(vet[pp]>vet[pp+1])
            {
                scambio(&vet[pp], &vet[pp+1]);
            }
            else
            {
                vetOrdinato++;
            }
        }
        if(vetOrdinato==3-qq)
        {
            break;
        }
    }
    return vet[3];
}

//funzione enigma
int Enigma(int num,int numrot1,int numrot2,int numrot3, char rotorSettingCharInput[], char messaggioDaCifrare[],char messaggioCifrato[])
{
    int enigma[num];
    int ii, kk;
    int numrot[3];
    numrot[0]=numrot1;
    numrot[1]=numrot2;
    numrot[2]=numrot3;
    sort(numrot);
    
    FILE *fin;
    fin=fopen("/Users/giancarlobonzi/Desktop/input.txt","r");
    if(fin == NULL)
    {
        printf("Impossibile aprire il file\n");
        return 0;
    }
    int rotorSetting[27][3];
    for(ii=0;ii<27;ii++)
    {
        fscanf(fin,"%d",&rotorSetting[ii][numrot[0]]);
    }
    for(ii=0;ii<27;ii++)
    {
        fscanf(fin,"%d",&rotorSetting[ii][numrot[1]]);
    }
    for(ii=0;ii<27;ii++)
    {
        fscanf(fin,"%d",&rotorSetting[ii][numrot[2]]);
    }
    fclose(fin);
    int reflectorn[] = {24,17,20,7,16,18,11,3,15,23,13,6,14,10,12,8,4,1,5,25,2,22,21,9,0,19};
    
    //converto le lettere in numeri
    for(ii=0;ii<num;ii++)
    {
        for(kk=0;kk<26;kk++)
        {
            if(messaggioDaCifrare[ii]==alphabet[kk])
            {
                enigma[ii]=kk;
                break;
            }
        }
    }
    
    
    //converto le lettere in numeri
    int rotorSettingInput[4];
    for(ii=0;ii<3;ii++)
    {
        for(kk=0;kk<26;kk++)
        {
            if(rotorSettingCharInput[ii]==alphabet[kk])
            {
                rotorSettingInput[ii]=kk;
                break;
            }
        }
    }
    rotorSettingInput[3]='\0';
    
    //inizio la cifratura tenendo conto del rotorSetting
    for(ii=0;ii<num;ii++)
    {
        rotorSettingInput[2]++;
        rotorSettingInput[2]=rotorSettingInput[2]%26;
        if(rotorSettingInput[2]==(rotorSetting[26][numrot3]+1))
        {
            rotorSettingInput[1]++;
            rotorSettingInput[1]=rotorSettingInput[1]%26;
        }
        if(rotorSettingInput[1]==rotorSetting[26][numrot2])
        {
            rotorSettingInput[1]++;
            rotorSettingInput[1]=rotorSettingInput[1]%26;
        }
        if(rotorSettingInput[1]==(rotorSetting[26][numrot2]+1))
        {
            rotorSettingInput[0]++;
            rotorSettingInput[0]=rotorSettingInput[0]%26;
        }
        
        //tutte le volte che entra una lettera il rotore scatta avanti di 1
        enigma[ii]+=rotorSettingInput[2];
        enigma[ii]=((enigma[ii] % 26)+26)%26;
        enigma[ii]=rotorSetting[enigma[ii]][numrot3];
        enigma[ii]=enigma[ii]-rotorSettingInput[2]+rotorSettingInput[1];
        enigma[ii]=((enigma[ii] % 26)+26)%26;
        enigma[ii]=rotorSetting[enigma[ii]][numrot2];
        enigma[ii]=enigma[ii]-rotorSettingInput[1]+rotorSettingInput[0];
        enigma[ii]=((enigma[ii] % 26)+26)%26;
        enigma[ii]=rotorSetting[enigma[ii]][numrot1];
        enigma[ii]=enigma[ii]-rotorSettingInput[0];
        enigma[ii]=((enigma[ii] % 26)+26)%26;
        enigma[ii]=reflectorn[enigma[ii]];
        
        //ritorno
        enigma[ii]=enigma[ii]+rotorSettingInput[0];
        enigma[ii]=((enigma[ii] % 26)+26)%26;
        for(kk=0;kk<26;kk++)
        {
            if(enigma[ii]==rotorSetting[kk][numrot1])
            {
                enigma[ii]=kk;
                break;
            }
        }
        
        enigma[ii]=enigma[ii]-rotorSettingInput[0]+rotorSettingInput[1];
        enigma[ii]=((enigma[ii] % 26)+26)%26;
        for(kk=0;kk<26;kk++)
        {
            if(enigma[ii]==rotorSetting[kk][numrot2])
            {
                enigma[ii]=kk;
                break;
            }
        }
        
        enigma[ii]=enigma[ii]-rotorSettingInput[1]+rotorSettingInput[2];
        enigma[ii]=((enigma[ii] % 26)+26)%26;
        for(kk=0;kk<26;kk++)
        {
            if(enigma[ii]==rotorSetting[kk][numrot3])
            {
                enigma[ii]=kk;
                break;
            }
        }
        enigma[ii]=enigma[ii]-rotorSettingInput[2];
        enigma[ii]=((enigma[ii] % 26)+26)%26;
    }
    for(ii=0;ii<num;ii++)
    {
        messaggioCifrato[ii]=alphabet[enigma[ii]];
    }
    messaggioCifrato[num]='\0';
    fclose(fin);
    return 0;
}


// funzione Quadgramma
int Quadgramma(char quadgrammaInIngresso[],long *probDelQuadgramma)
{
    *probDelQuadgramma=0;
    int notFound=1;
    long int iii=0;
    while(notFound && iii<numMaxQuad && *probDelQuadgramma<=13168375)
    {
        notFound=strcmp(quadgrammaInIngresso, englishQuad[iii]);
        if (notFound==0)
        {
            *probDelQuadgramma=englishQuadProb[iii];
        }
        iii++;
    }
    
    //si esce dal while, se si trova il quadrgramma quadgrammaInIngresso
    //oppure   se iii arriva al massimo numero di quadgrammi nel FILE
    //oppure se la probabilità associata a quadgrammaInIngresso è un numero grandissimo senza senso
    return 1;
}

int InitQuadgramma()
{
    int found=0;
    long int iii=0, probability=0;
    char quadgrams[5];
    FILE *quad;
    quad=fopen("/Users/giancarlobonzi/Desktop/provaser/provaser/englishQuadgrams.txt","r");
    if(quad == NULL)
    {
        printf("Impossibile aprire il file\n");
        return 0;
    }
    while(found!=EOF)
    {
        found=fscanf(quad,"%s %ld \n", quadgrams, &probability);
        if(found>1)
        {
            englishQuadProb[iii]=probability;
            strcpy(englishQuad[iii],quadgrams);
            iii++;
            numMaxQuad=iii;
        }
        
    }
    fclose(quad);
    return 1;
}

//main
int main()
{
    int num, numrot1, numrot2, numrot3, ii, kk, jj, controllo;
    printf("Inserisci quante lettere vuoi cifrare (max 127):\t");
    scanf("%d",&num);
    char daCifrare[N_MAX_STRINGA_CRITT], cifrato[N_MAX_STRINGA_CRITT];
    
    //alfabeto standard 26 lettere
    //alphabet={'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'};
    alphabet[0]='A';
    alphabet[1]='B';
    alphabet[2]='C';
    alphabet[3]='D';
    alphabet[4]='E';
    alphabet[5]='F';
    alphabet[6]='G';
    alphabet[7]='H';
    alphabet[8]='I';
    alphabet[9]='J';
    alphabet[10]='K';
    alphabet[11]='L';
    alphabet[12]='M';
    alphabet[13]='N';
    alphabet[14]='O';
    alphabet[15]='P';
    alphabet[16]='Q';
    alphabet[17]='R';
    alphabet[18]='S';
    alphabet[19]='T';
    alphabet[20]='U';
    alphabet[21]='V';
    alphabet[22]='W';
    alphabet[23]='X';
    alphabet[24]='Y';
    alphabet[25]='Z';
    
    InitQuadgramma();
    FILE *fin;
    fin=fopen("/Users/giancarlobonzi/Desktop/input.txt","r");
    if(fin == NULL)
    {
        printf("Impossibile aprire il file\n");
        return 0;
    }
    printf("Qual è l'ordine dei rotori?\n");
    scanf("%d",&numrot1);
    scanf("%d",&numrot2);
    scanf("%d",&numrot3);
    

    
    //prendo in input come lettere il rotorsetting
    printf("Inserisci il codice in lettere del rotorsetting: (RICORDATI DI USARE L'ALFABETO MAIUSCOLO)\n");
    char seqInizioRotori[4];
    ii=0;
    controllo=0;
    do {
        scanf("%c",&seqInizioRotori[ii]);
        for(kk=0;kk<26;kk++)
        {
            if(seqInizioRotori[ii]==alphabet[kk])
            {
                controllo++;
                ii++;
                break;
            }
        }
    } while (controllo<3);
    seqInizioRotori[3]='\0';
    
    controllo=0;
    ii=0;
    printf("Enigma correttamente impostato!\n");
    printf("Inserisci il codice da cifrare: (RICORDATI DI USARE L'ALFABETO MAIUSCOLO)\n");
    do {
        scanf("%c",&daCifrare[ii]);
        for(kk=0;kk<26;kk++)
        {
            if(daCifrare[ii]==alphabet[kk])
            {
                controllo++;
                ii++;
                break;
            }
        }
    } while (controllo<num);
    daCifrare[num]='\0';
    
    Enigma(num, numrot1, numrot2, numrot3, seqInizioRotori, daCifrare, cifrato);
    for(ii=0;ii<num;ii++)
    {
        printf("%c",cifrato[ii]);
    }
    printf("\nOra calcolo la chiave che cifra il testo crittato in una frase in chiaro di senso compiuto (nella lingua inglese):\n");
    
    char newCifrato[N_MAX_STRINGA_CRITT],newCifratoMax[N_MAX_STRINGA_CRITT],quadDaCercare[N_MAX_STRINGA_CRITT];
    newCifrato[num]='\0', newCifratoMax[num]='\0';
    int rot1Max=0, rot2Max=0, rot3Max=0, qqq,r1,r2,r3;
    int letteraRotSet1, letteraRotSet2, letteraRotSet3, letteraRotSetMax1=0, letteraRotSetMax2=0, letteraRotSetMax3=0;
    long probMax=0, probDelQuadgramma, score=0;
    
    
        for (r1=1;r1<4;r1++)
        {
            for (r2=1;r2<4;r2++)
            {
                for (r3=1;r3<4;r3++)
                {
                    if (r2!=r1 && r3!=r1 && r3!=r2)
                    {
                        for(letteraRotSet1=0;letteraRotSet1<26;letteraRotSet1++)
                        {
                            for(letteraRotSet2=0;letteraRotSet2<26;letteraRotSet2++)
                            {
                                for(letteraRotSet3=0;letteraRotSet3<26;letteraRotSet3++)
                                {
                                    seqInizioRotori[0]=alphabet[letteraRotSet1];
                                    seqInizioRotori[1]=alphabet[letteraRotSet2];
                                    seqInizioRotori[2]=alphabet[letteraRotSet3];
                                    score=0;
                                    for(jj=0;jj<(num-3);jj++)
                                    {
                                        //adesso si prendono 4 lettere alla volta da cifrato, e si vede se enigma(queste4lettere è una parola inglese4
                                        strcpy(quadDaCercare,cifrato+jj);
                                        quadDaCercare[4]='\0';
                                        Enigma(4, r1, r2, r3, seqInizioRotori, quadDaCercare, newCifrato);
                                        Quadgramma(newCifrato, &probDelQuadgramma);
                                        score+=probDelQuadgramma;
                                        if (score>probMax)
                                        {
                                            rot1Max=r1;
                                            rot2Max=r2;
                                            rot3Max=r3;
                                            letteraRotSetMax1=letteraRotSet1;
                                            letteraRotSetMax2=letteraRotSet2;
                                            letteraRotSetMax3=letteraRotSet3;
                                            probMax=score;
                                            for (qqq=0;qqq<4;qqq++)
                                            {
                                                newCifratoMax[qqq+jj]=newCifrato[qqq];
                                            }
                                            newCifratoMax[jj+4]='\0';
                                            printf("Intermedio: %s , %ld\n %d, %d, %d\n %c, %c, %c\n", newCifratoMax, probMax, rot1Max, rot2Max, rot3Max, alphabet[letteraRotSetMax1], alphabet[letteraRotSetMax2], alphabet[letteraRotSetMax3]);
                                        
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    printf("Finale: %s , %ld\n %d, %d, %d\n %c, %c, %c\n", newCifratoMax, probMax, rot1Max, rot2Max, rot3Max, alphabet[letteraRotSetMax1], alphabet[letteraRotSetMax2], alphabet[letteraRotSetMax3]);
    return 0;
}

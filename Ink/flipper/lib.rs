#![cfg_attr(not(feature = "std"), no_std, no_main)]

//use ink_lang as ink;
#[ink::contract]
mod flipper {
    use ink::prelude::string::String;
    use scale_info::prelude::format;
    use ink::prelude::string::ToString;
    use ink::prelude::vec::Vec;
    use scale::{Decode, Encode};
    //use serde::{Serialize, Deserialize};

    #[derive(Encode, Decode, PartialEq, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Genero {
        Acao,
        Animacao,
        Comedia,
        Drama,
        Gospel,
        Suspense,
        Outros,
    }
    
    #[derive(Encode, Decode, PartialEq, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Filme {
        id: u32,
        nome: String,
        bilhetes_vendidos: u32,
        ano_lancamento: u32, 
        mes_lancamento: u8,
        dia_lancamento: u8,
        genero: Genero,
    }
    #[derive(Encode, Decode, PartialEq, Debug, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct nomesEIds{
        nome: String,
        id: u32,
    }
    #[ink(storage)]
    pub struct Flipper {
        lista_filmes: Vec<Filme>,
        lista_nomes: Vec<nomesEIds>,
    }

    impl Flipper {
        /// Construtor que inicializa um filme de exemplo.
        #[ink(constructor)]
        pub fn new_with_example() -> Self {
            let filme_exemplo = Filme {
                id: 1,
                nome: String::from("Filme Exemplo"),
                bilhetes_vendidos: 1000,
                ano_lancamento: 2025,
                mes_lancamento: 1,
                dia_lancamento: 1,
                genero: Genero::Acao,
            };
            let nome_e_id = nomesEIds {
                nome: filme_exemplo.nome.clone(),
                id: 1,
            };
            Self {
                lista_nomes: Vec::from([nome_e_id]),
                lista_filmes: Vec::from([filme_exemplo]),
            }
        }
        #[ink(constructor)]
        pub fn new_with_custom(
            nome: String,
            bilhetes_vendidos: u32,
            ano_lancamento: u32,
            mes_lancamento: u8,
            dia_lancamento: u8,
            genero: Genero) -> Result<Self,String> {
            let filme_exemplo = Filme {
                id : 0,
                nome,
                bilhetes_vendidos,
                ano_lancamento,
                mes_lancamento,
                dia_lancamento,
                genero,
            };
            let nome_e_id = nomesEIds {
                nome: filme_exemplo.nome.clone(),
                id: 1,
            };
            let instance = Self {
                lista_nomes: Vec::from([nome_e_id]),
                lista_filmes: Vec::from([filme_exemplo.clone()]),
            };
            if filme_exemplo.nome == "" {
                return Err(String::from("Por favor, insira um nome para o filme"));
            }
            if let Err(e) = instance.checa_data(filme_exemplo.ano_lancamento,filme_exemplo.mes_lancamento,filme_exemplo.dia_lancamento) {
                return Err(e);
            }

            Ok(instance)
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default()  -> Self {
            Self {
                lista_filmes: Vec::new(),
                lista_nomes: Vec::new(),
            }
        }

        /// A message that can be called on instantiated contracts.
        #[ink(message)]
        pub fn add_filme(
            &mut self,
            nome: String,
            bilhetes_vendidos: u32,
            ano_lancamento: u32,
            mes_lancamento: u8,
            dia_lancamento: u8,
            genero: Genero,
        ) -> Result<(), String> {
            let id = self.calculaId();
            let novo_filme = Filme {
                id,
                nome,
                bilhetes_vendidos,
                ano_lancamento,
                mes_lancamento,
                dia_lancamento,
                genero,
            };
            if novo_filme.nome == "" {
                return Err(String::from("Por favor, insira um nome para o filme"));
            }
            if let Err(e) = self.checa_data(ano_lancamento, mes_lancamento, dia_lancamento) { 
                return Err(e);
            }

            if  self.checa_nome_unico(&novo_filme.nome){
                return Err( "Esse nome já existe no sistema!".to_string(),);
            }

            self.lista_nomes.push(nomesEIds{
                nome: novo_filme.nome.clone(),
                id: id,
            });
            self.lista_filmes.push(novo_filme);
          
            Ok(())
        }

        #[ink(message)]
        pub fn get_lista_filmes(&self) -> Vec<Filme> {
            self.lista_filmes.clone()
        }
        #[ink(message)]
        pub fn delete_filme(
            &mut self,
            id: u32,
        ) -> Result<(), String> {

            let ind = match self.get_index_filme(id) {
                Ok(num) => num,
                Err(e) => {
                    return Err(e);
                },
            };
          
            self.lista_nomes.remove(ind);
            self.lista_filmes.remove(ind);
            Ok(())
        }
        #[ink(message)]
        pub fn update_filme(
            &mut self,
            id_filme_a_atualizar:u32,
            novo_nome_filme:String,
            bilhetes_vendidos: u32,
            ano_lancamento: u32,
            mes_lancamento: u8,
            dia_lancamento: u8,
            genero: Genero,
        ) -> Result<(), String> {
            let ind = match self.get_index_filme(id_filme_a_atualizar) {
                Ok(num) => num,
                Err(e) => {
                    return Err(e);
                },
            };
            
            if novo_nome_filme != "" {
                if  self.checa_nome_unico(&novo_nome_filme){
                    return Err( "Esse nome já existe no sistema!".to_string(),);
                }
                else{
                    self.lista_nomes[ind].nome = novo_nome_filme.clone();
                    self.lista_filmes[ind].nome = novo_nome_filme;
                }
            }
            if bilhetes_vendidos != 0 {
                self.lista_filmes[ind].bilhetes_vendidos = bilhetes_vendidos;
            }

            if ano_lancamento != 0 || mes_lancamento != 0 || dia_lancamento != 0 {
                if let Err(e) = self.checa_data(ano_lancamento, mes_lancamento, dia_lancamento) { 
                    return Err(e);
                }
                else{
                    self.lista_filmes[ind].ano_lancamento = ano_lancamento;
                    self.lista_filmes[ind].mes_lancamento = mes_lancamento;
                    self.lista_filmes[ind].dia_lancamento = dia_lancamento;
                }
            }
            self.lista_filmes[ind].genero = genero;
            Ok(())
        }


        //Validadores
        pub fn get_index_filme(&self, id: u32) -> Result<usize, String>{
            if self.lista_nomes.len() == 0 {
                return Err(String::from("Não existem filmes no sistema!"));
            }

            for (index, filme) in self.lista_nomes.iter().enumerate(){
                if filme.id == id {
                    return Ok(index);
                }
            }
            return Err(format!("Não existe um filme com esse nome!\nFilmes disponíveis: {:#?}", &self.lista_nomes));
        }

        pub fn checa_nome_unico(&self, nome_f: &str) -> bool{
            if self.lista_nomes.len() == 0 {
                return false;
            }

            for i in &self.lista_nomes{
                if i.nome == nome_f {
                    return true;
                }
            }
            return false
        }

        pub fn checa_data(&self, ano: u32, mes: u8, dia: u8) ->Result<(), String> {
            if ano < 2000 || ano > 2025 {
                return Err(String::from("Por favor, insira um ano válido (Entre 2000 e 2025)!"));
            }
            if mes < 1 || mes > 12 {
                return Err(String::from("Por favor, insira um mês válido (Entre 1 e 12)!"));
            }
            if dia < 1 || dia > 31 {
                 return Err(String::from("Por favor, insira um dia válido (Entre 1 e 31)!"));
            }

            match mes {
                4 | 6 | 9 | 11 if dia > 30 => return Err(String::from("Data inválida!")),
                2 => {
                    if ano % 4 == 0 && (ano % 100 != 0 || ano % 400 == 0) {
                        if dia > 29 {
                            return Err(String::from("Data inválida!"))
                        }
                    } else if dia > 28{
                       return Err(String::from("Data inválida!")) 
                    }
                }
                _ => return Ok(()),
            }
            return Ok(())
        }

        pub fn calculaId(&self) -> u32{
            if self.lista_filmes.is_empty() {
                return 0;
            }

            if let Some(last_index) = self.lista_filmes.len().checked_sub(1) {
                if let Some(new_id) = self.lista_nomes[last_index].id.checked_add(1) {
                    return new_id;
                }
            }

            0
        }
    }
   
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get_lista_filmes().is_empty(), true);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn with_example_works() {
            let flipper = Flipper::new_with_example();
            let filme_exemplo = flipper.get_lista_filmes()[0].clone();

            assert_eq!(filme_exemplo.nome, "Filme Exemplo");
            assert_eq!(filme_exemplo.bilhetes_vendidos, 1000);
            assert_eq!(filme_exemplo.ano_lancamento, 2025);
            assert_eq!(filme_exemplo.mes_lancamento, 1);
            assert_eq!(filme_exemplo.dia_lancamento, 1);
            assert_eq!(filme_exemplo.genero, Genero::Acao);
        }
        #[ink::test]
        fn with_custom_works() {
            let nome_f = String::from("Novo filme");
            let bilhetes_v = 200;
            let ano_l = 2005;
            let mes_l = 10;
            let dia_l = 2;
            let gen = Genero::Acao;

            let flipper = Flipper::new_with_custom(nome_f.clone(), bilhetes_v, ano_l, mes_l, dia_l, gen.clone())
                .expect("Falha ao criar Flipper");
            let filme_exemplo = flipper.get_lista_filmes()[0].clone();


            assert_eq!(filme_exemplo.nome, nome_f);
            assert_eq!(filme_exemplo.bilhetes_vendidos, bilhetes_v);
            assert_eq!(filme_exemplo.ano_lancamento, ano_l);
            assert_eq!(filme_exemplo.mes_lancamento, mes_l);
            assert_eq!(filme_exemplo.dia_lancamento, dia_l);
            assert_eq!(filme_exemplo.genero, gen);
        }
        #[ink::test]
        fn with_custom_no_name() {
            let nome_f = String::from("");
            let bilhetes_v = 200;
            let ano_l = 2005;
            let mes_l = 10;
            let dia_l = 2;
            let gen = Genero::Acao;

            let _ = match Flipper::new_with_custom(nome_f.clone(), bilhetes_v, ano_l, mes_l, dia_l, gen.clone()){
                Ok(_) =>Err(String::from("Não deveria ser possível aceitar filme sem nome")),
                Err(e) =>{
                    if e.contains("Por favor, insira um nome para o filme"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
        }
        #[ink::test]
        fn with_custom_invalid_date() {
            let nome_f = String::from("Filme inedito!");
            let bilhetes_v = 200;
            let ano_l = 2008;
            let mes_l = 2;
            let dia_l = 31;
            let gen = Genero::Acao;

            let _ = match Flipper::new_with_custom(nome_f.clone(), bilhetes_v, ano_l, mes_l, dia_l, gen.clone()){
                Ok(_) =>Err(String::from("Não deveria ser possível aceitar filme com data inválida")),
                Err(e) =>{
                    if e.contains("Data inválida!"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
        }
        #[ink::test]
        fn creating_valid_movie() {
            let mut flipper = Flipper::default();
            let _ = match flipper.add_filme(String::from("Filme"), 2000, 2005, 10, 10, Genero::Acao){
                Ok(_) =>Ok(()),
                Err(e) => Err(e),
            };
        }
        #[ink::test]
        fn creating_invalid_name_movie() {
            let mut flipper = Flipper::new_with_example();
            let _ = match flipper.add_filme(String::from("Filme Exemplo"), 2000, 2005, 10, 10, Genero::Acao){
                Ok(_) =>Err(String::from("Não deveria ser possível aceitar um nome de filme já existente!")),
                Err(e) =>{
                    if e.contains("Esse nome já existe no sistema!"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
        }
        #[ink::test]
        fn creating_with_no_name_movie() {
            let mut flipper = Flipper::new_with_example();
            let _ = match flipper.add_filme(String::from(""), 2000, 2005, 10, 10, Genero::Acao){
                Ok(_) =>Err(String::from("Não deveria ser possível aceitar filme sem nome")),
                Err(e) =>{
                    if e.contains("Por favor, insira um nome para o filme"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
        }
        #[ink::test]
        fn creating_invalid_date_movie() {
            let mut flipper = Flipper::default();
            let _ = match flipper.add_filme(String::from("Filme Exemplo"), 2000, 1500, 10, 10, Genero::Acao){
                Ok(_) =>Err(String::from("Não deveria ser possível aceitar um filme desse ano!")),
                Err(e) =>{
                    if e.contains("Por favor, insira um ano válido (Entre 2000 e 2025)!"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
            let _ = match flipper.add_filme(String::from("Filme Exemplo"), 2000, 2010, 18, 10, Genero::Acao){
                Ok(_) =>Err(String::from("Não deveria ser possível aceitar um filme com mês inválido!")),
                Err(e) =>{
                    if e.contains("Por favor, insira um mês válido (Entre 1 e 12)!"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
            let _ = match flipper.add_filme(String::from("Filme Exemplo"), 2000, 2010, 10, 50, Genero::Acao){
                Ok(_) =>Err(String::from("Não deveria ser possível aceitar um filme com dia inválido!")),
                Err(e) =>{
                    if e.contains("Por favor, insira um dia válido (Entre 1 e 31)!"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
            let _ = match flipper.add_filme(String::from("Filme Exemplo"), 2000, 2010, 2, 31, Genero::Acao){
                Ok(_) =>Err(String::from("Não deveria ser possível aceitar um filme com uma data inválida!")),
                Err(e) =>{
                    if e.contains("Data inválida!"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
        }
        #[ink::test]
        fn creating_multiple_movies() {
            let mut flipper = Flipper::default();
    
            let _ = flipper.add_filme(String::from("Filme 1"), 500, 2010, 5, 10, Genero::Acao);
            let _ = flipper.add_filme(String::from("Filme 2"), 1500, 2015, 7, 15, Genero::Comedia);
            let _ = flipper.add_filme(String::from("Filme 3"), 2000, 2020, 12, 1, Genero::Drama);

            let filmes = flipper.get_lista_filmes();
            assert_eq!(filmes.len(), 3);
        }

        #[ink::test]
        fn deleating_valid_movie() {
            let mut flipper = Flipper::new_with_example();
            let _ = match flipper.delete_filme(String::from("Filme Exemplo")){
                Ok(_) =>Ok(()),
                Err(e) => Err(e),
            };
        }
        #[ink::test]
        fn deleating_without_movie() {
            let mut flipper = Flipper::default();
            let _ = match flipper.delete_filme(String::from("Filme Exemplo")){
                Ok(_) =>Ok(()),
                Err(e) => {
                    if e.contains("Não existem filmes no sistema!"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
        }
        #[ink::test]
        fn deleating_invalid_movie() {
            let mut flipper = Flipper::new_with_example();
            let _ = match flipper.delete_filme(String::from("Outro Filme Exemplo que não existe!")){
                Ok(_) =>Ok(()),
                Err(e) => {
                    if e.contains("Não existe um filme com esse nome!\nFilmes disponíveis: "){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
        }
        #[ink::test]
        fn deleting_all_movies() {
            let mut flipper = Flipper::new_with_example();
    
            let _ = flipper.delete_filme(String::from("Filme Exemplo"));
    
            assert!(flipper.get_lista_filmes().is_empty());
        }

        #[ink::test]
        fn updating_valid_movie() {
            let mut flipper = Flipper::new_with_example();
            let _ = match flipper.update_filme(String::from("Filme Exemplo"), String::from("Novo nome"), 0, 0, 0, 0, Genero::Acao){
                //Atualizando apenas o nome
                Ok(_) =>Ok(()),
                Err(e) => Err(e),
            };
            let _ = match flipper.update_filme(String::from("Novo nome"), String::from(""), 5000, 0, 0, 0, Genero::Acao){
                //Atualizando apenas bilhetes vendidos
                Ok(_) =>Ok(()),
                Err(e) => Err(e),
            };
            let _ = match flipper.update_filme(String::from("Novo nome"), String::from(""), 0, 2020, 12, 30, Genero::Acao){
                //Atualizando apenas a data
                Ok(_) =>Ok(()),
                Err(e) => Err(e),
            };
            let _ = match flipper.update_filme(String::from("Novo nome"), String::from("Novissimo nome"), 3000, 2012, 10, 20, Genero::Comedia){
                //Atualizando todos os atributos simultaneamente
                Ok(_) =>Ok(()),
                Err(e) => Err(e),
            };
        }
         #[ink::test]
        fn updating_without_movie() {
            let mut flipper = Flipper::default();
            let _ = match flipper.update_filme(String::from("Filme Exemplo"), String::from(""), 5000, 0, 0, 0, Genero::Acao){
                Ok(_) =>Ok(()),
                Err(e) => {
                    if e.contains("Não existem filmes no sistema!"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
        }
        #[ink::test]
        fn updating_no_existing_movie() {
            let mut flipper = Flipper::new_with_example();
            let _ = match flipper.update_filme(String::from("Filme Exemplo não existente"), String::from(""), 5000, 0, 0, 0, Genero::Acao){
                Ok(_) =>Ok(()),
                Err(e) => {
                    if e.contains("Não existe um filme com esse nome!\nFilmes disponíveis: "){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
        }
        #[ink::test]
        fn updating_invalid_name() {
            let mut flipper = Flipper::new_with_example();
            let _ = match flipper.update_filme(String::from("Filme Exemplo"), String::from("Filme Exemplo"), 0, 0, 0, 0, Genero::Acao){
                Ok(_) =>Ok(()),
                Err(e) => {
                    if e.contains("Esse nome já existe no sistema!"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
        }
        #[ink::test]
        fn updating_invalid_date() {
            let mut flipper = Flipper::new_with_example();
            let _ = match flipper.update_filme(String::from("Filme Exemplo"), String::from("Outro Filme Exemplo"), 2000, 1500, 10, 10, Genero::Acao){
                Ok(_) =>Err(String::from("Não deveria ser possível aceitar um filme desse ano!")),
                Err(e) =>{
                    if e.contains("Por favor, insira um ano válido (Entre 2000 e 2025)!"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
            let _ = match flipper.update_filme(String::from("Filme Exemplo"), String::from("Outro Filme Exemplo"), 2000, 2010, 18, 10, Genero::Acao){
                Ok(_) =>Err(String::from("Não deveria ser possível aceitar um filme com mês inválido!")),
                Err(e) =>{
                    if e.contains("Por favor, insira um mês válido (Entre 1 e 12)!"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
            let _ = match flipper.update_filme(String::from("Filme Exemplo"), String::from("Outro Filme Exemplo"), 2000, 2010, 10, 50, Genero::Acao){
                Ok(_) =>Err(String::from("Não deveria ser possível aceitar um filme com dia inválido!")),
                Err(e) =>{
                    if e.contains("Por favor, insira um dia válido (Entre 1 e 31)!"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
            let _ = match flipper.update_filme(String::from("Filme Exemplo"), String::from("Outro Filme Exemplo"),2000, 2010, 2, 31, Genero::Acao){
                Ok(_) =>Err(String::from("Não deveria ser possível aceitar um filme com dia inválido!")),
                Err(e) =>{
                    if e.contains("Data inválida!"){
                        Ok(())
                    }else{
                        Err(e)
                    }
                },
            };
        }
        #[ink::test]
        fn updating_just_gender() {
            let mut flipper = Flipper::new_with_example();
    
            let filme_anterior = flipper.get_lista_filmes()[0].clone();
    
            let _ = flipper.update_filme(String::from("Filme Exemplo"), String::from(""), 0, 0, 0, 0, Genero::Outros);
    
            let filme_atualizado = &flipper.get_lista_filmes()[0];
    
            assert_eq!(filme_atualizado.nome, filme_anterior.nome);
            assert_eq!(filme_atualizado.bilhetes_vendidos, filme_anterior.bilhetes_vendidos);
            assert_eq!(filme_atualizado.ano_lancamento, filme_anterior.ano_lancamento);
            assert_eq!(filme_atualizado.genero, Genero::Outros); 
        }

    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = FlipperRef::default();

            // When
            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Flipper>();

            // Then
            let get = call_builder.get_lista_filmes();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value().is_empty(), true));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn example_movie_works_eTwoe(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = FlipperRef::new_with_example();

            // When
            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Flipper>();

            // Then
            let get = call_builder.get_lista_filmes();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            let filmes = get_result.return_value();
    
            assert_eq!(filmes.len(), 1);

            let filme_exemplo = &filmes[0];

            assert_eq!(filme_exemplo.nome, "Filme Exemplo");
            assert_eq!(filme_exemplo.bilhetes_vendidos, 1000);
            assert_eq!(filme_exemplo.ano_lancamento, 2025);
            assert_eq!(filme_exemplo.mes_lancamento, 1);
            assert_eq!(filme_exemplo.dia_lancamento, 1);
            assert_eq!(filme_exemplo.genero, Genero::Acao);

            Ok(())
        }
        #[ink_e2e::test]
        async fn example_movie_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let nome_f = String::from("Novo filme");
            let bilhetes_v = 200;
            let ano_l = 2005;
            let mes_l = 10;
            let dia_l = 2;
            let gen = Genero::Acao;

            let mut constructor = FlipperRef::new_with_custom(nome_f.clone(), bilhetes_v, ano_l, mes_l, dia_l, gen.clone());

            // When
            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Flipper>();

            // Then
            let get = call_builder.get_lista_filmes();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            let filmes = get_result.return_value();
    
            assert_eq!(filmes.len(), 1);

            let filme_exemplo = &filmes[0];

            assert_eq!(filme_exemplo.nome, nome_f);
            assert_eq!(filme_exemplo.bilhetes_vendidos, bilhetes_v);
            assert_eq!(filme_exemplo.ano_lancamento, ano_l);
            assert_eq!(filme_exemplo.mes_lancamento, mes_l);
            assert_eq!(filme_exemplo.dia_lancamento, dia_l);
            assert_eq!(filme_exemplo.genero, gen);

            Ok(())
        }
       #[ink_e2e::test]
       async fn create_valid_movie(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given:
            let mut constructor = FlipperRef::default();

            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");

            let mut call_builder = contract.call_builder::<Flipper>();

            // Then: 
            let get = call_builder.get_lista_filmes();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert!(get_result.return_value().is_empty());

            // When: 
            let add_filme = call_builder.add_filme(
                String::from("Filme"), 
                2000, 
                2005, 
                10, 
                10, 
                Genero::Acao
            );

            client.call(&ink_e2e::alice(), &add_filme).submit().await?; 

            // Then: 
            let get_after_add = call_builder.get_lista_filmes();
            let get_result_after_add = client.call(&ink_e2e::alice(), &get_after_add).dry_run().await?;
    
            let filmes = get_result_after_add.return_value();
            assert_eq!(filmes.len(), 1);

            Ok(())
        }
       #[ink_e2e::test]
       async fn create_invalid_name_movie(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given: 
            let mut constructor = FlipperRef::new_with_example();

            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");

            let mut call_builder = contract.call_builder::<Flipper>();

            // Then: 
            let get = call_builder.get_lista_filmes();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            let filmes = get_result.return_value();
            assert_eq!(filmes.is_empty(), false);

            let nome_filme_existente = filmes[0].nome.clone(); 
            // When: 
            let add_filme = call_builder.add_filme(
                nome_filme_existente.clone(), 
                2000, 
                2005, 
                10, 
                10, 
                Genero::Acao
            );

            let err = client.call(&ink_e2e::alice(), &add_filme).submit().await;
            match err{
                Ok(_) =>panic!("Não deveria ser possível aceitar um nome de filme já existente!"),
                Err(e) =>{
                    assert!(e.to_string().contains("Esse nome já existe no sistema!"),
                    "Erro inesperado: {:?}",e);
                },
            };

            // Then: 
            let get_after_add = call_builder.get_lista_filmes();
            let get_result_after_add = client.call(&ink_e2e::alice(), &get_after_add).dry_run().await?;
    
            let filmes = get_result_after_add.return_value();
            assert_eq!(filmes.len(), 1);

            Ok(())
        }

        #[ink_e2e::test]
       async fn create_invalid_date_movie(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given: 
            let mut constructor = FlipperRef::new_with_example();

            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");

            let mut call_builder = contract.call_builder::<Flipper>();

            // Then: 
            let get = call_builder.get_lista_filmes();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            let filmes = get_result.return_value();
            assert_eq!(filmes.is_empty(), false);

            // When: 
            let add_filme = call_builder.add_filme(
                String::from("Outro nome de filme"), 
                2000, 
                2005, 
                2, 
                31, 
                Genero::Acao
            );

            let err = client.call(&ink_e2e::alice(), &add_filme).submit().await;
            match err{
                Ok(_) =>panic!("Não deveria ser possível aceitar um filme com data invalida"),
                Err(e) =>{
                    assert!(e.to_string().contains("Data inválida!"),
                    "Erro inesperado: {:?}",e);
                },
            };

            // Then: 
            let get_after_add = call_builder.get_lista_filmes();
            let get_result_after_add = client.call(&ink_e2e::alice(), &get_after_add).dry_run().await?;
    
            let filmes = get_result_after_add.return_value();
            assert_eq!(filmes.len(), 1);

            Ok(())
        }
        #[ink_e2e::test]
       async fn update_valid_movie(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given:
            let mut constructor = FlipperRef::new_with_example();

            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");

            let mut call_builder = contract.call_builder::<Flipper>();

            // Then: 
            let get = call_builder.get_lista_filmes();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            let filmes = get_result.return_value();
            assert_eq!(filmes.is_empty(), false);

            let nome_filme_existente = filmes[0].nome.clone(); 

            // When: 
            let add_filme = call_builder.update_filme(
                nome_filme_existente.clone(),
                String::from("Novo nome"),
                200, 
                2003, 
                10, 
                20, 
                Genero::Comedia
            );

            client.call(&ink_e2e::alice(), &add_filme).submit().await?; 

            // Then: 
            let get_after_add = call_builder.get_lista_filmes();
            let get_result_after_add = client.call(&ink_e2e::alice(), &get_after_add).dry_run().await?;
    
            let filmes = get_result_after_add.return_value();
            assert_eq!(filmes.len(), 1);

            let filme_atualizado = &filmes[0];

            // Verificamos se os dados do filme foram realmente atualizados
            assert_eq!(filme_atualizado.nome, "Novo nome");
            assert_eq!(filme_atualizado.bilhetes_vendidos, 200);
            assert_eq!(filme_atualizado.ano_lancamento, 2003);
            assert_eq!(filme_atualizado.mes_lancamento, 10);
            assert_eq!(filme_atualizado.dia_lancamento, 20);
            assert_eq!(filme_atualizado.genero, Genero::Comedia);

            Ok(())
        }
        #[ink_e2e::test]
       async fn update_invalid_name_movie(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given: 
            let mut constructor = FlipperRef::new_with_example();

            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");

            let mut call_builder = contract.call_builder::<Flipper>();

            // Then: 
            let get = call_builder.get_lista_filmes();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            let filmes = get_result.return_value();
            assert_eq!(filmes.is_empty(), false);

            let nome_filme_existente = filmes[0].nome.clone(); 
            // When: 
            let add_filme = call_builder.update_filme(
                nome_filme_existente.clone(), 
                nome_filme_existente.clone(), 
                2000, 
                2005, 
                10, 
                10, 
                Genero::Acao
            );

            let err = client.call(&ink_e2e::alice(), &add_filme).submit().await;
            match err{
                Ok(_) =>panic!("Não deveria ser possível aceitar um nome de filme já existente!"),
                Err(e) =>{
                    assert!(e.to_string().contains("Esse nome já existe no sistema!"),
                    "Erro inesperado: {:?}",e);
                },
            };

            // Then: 
            let get_after_add = call_builder.get_lista_filmes();
            let get_result_after_add = client.call(&ink_e2e::alice(), &get_after_add).dry_run().await?;
    
            let filmes_apos_operacao = get_result_after_add.return_value();
            let filme_atualizado = &filmes_apos_operacao[0];

            assert_eq!(filme_atualizado.nome, filmes[0].nome.clone());
            assert_eq!(filme_atualizado.bilhetes_vendidos, filmes[0].bilhetes_vendidos);
            assert_eq!(filme_atualizado.ano_lancamento, filmes[0].ano_lancamento);
            assert_eq!(filme_atualizado.mes_lancamento, filmes[0].mes_lancamento);
            assert_eq!(filme_atualizado.dia_lancamento, filmes[0].dia_lancamento);
            assert_eq!(filme_atualizado.genero, filmes[0].genero);
            Ok(())
        }
        #[ink_e2e::test]
       async fn update_invalid_date_movie(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given: 
            let mut constructor = FlipperRef::new_with_example();

            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");

            let mut call_builder = contract.call_builder::<Flipper>();

            // Then: 
            let get = call_builder.get_lista_filmes();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            let filmes = get_result.return_value();
            assert_eq!(filmes.is_empty(), false);

            let nome_filme_existente = filmes[0].nome.clone(); 
            // When: 
            let add_filme = call_builder.update_filme(
                nome_filme_existente.clone(), 
                nome_filme_existente.clone(), 
                2000, 
                2005, 
                2, 
                31, 
                Genero::Acao
            );

            let err = client.call(&ink_e2e::alice(), &add_filme).submit().await;
            match err{
                Ok(_) =>panic!("Não deveria ser possível aceitar filme com data inválida!"),
                Err(e) =>{
                    assert!(e.to_string().contains("Data inválida!"),
                    "Erro inesperado: {:?}",e);
                },
            };

            // Then: 
            let get_after_add = call_builder.get_lista_filmes();
            let get_result_after_add = client.call(&ink_e2e::alice(), &get_after_add).dry_run().await?;
    
            let filmes_apos_operacao = get_result_after_add.return_value();
            let filme_atualizado = &filmes_apos_operacao[0];

            assert_eq!(filme_atualizado.nome, filmes[0].nome.clone());
            assert_eq!(filme_atualizado.bilhetes_vendidos, filmes[0].bilhetes_vendidos);
            assert_eq!(filme_atualizado.ano_lancamento, filmes[0].ano_lancamento);
            assert_eq!(filme_atualizado.mes_lancamento, filmes[0].mes_lancamento);
            assert_eq!(filme_atualizado.dia_lancamento, filmes[0].dia_lancamento);
            assert_eq!(filme_atualizado.genero, filmes[0].genero);
            Ok(())
        }

        #[ink_e2e::test]
       async fn delete_valid_movie(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given:
            let mut constructor = FlipperRef::new_with_example();

            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");

            let mut call_builder = contract.call_builder::<Flipper>();

            // Then: 
            let get = call_builder.get_lista_filmes();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            let filmes = get_result.return_value();
            assert_eq!(filmes.is_empty(), false);

            let nome_filme_existente = filmes[0].nome.clone(); 

            // When: 
            let add_filme = call_builder.delete_filme(nome_filme_existente.clone());

            client.call(&ink_e2e::alice(), &add_filme).submit().await?; 

            // Then: 
            let get_after_add = call_builder.get_lista_filmes();
            let get_result_after_add = client.call(&ink_e2e::alice(), &get_after_add).dry_run().await?;
    
            let filmes = get_result_after_add.return_value();
            assert_eq!(filmes.len(), 0);

            Ok(())
        }
         #[ink_e2e::test]
       async fn delete_without_movie(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given: 
            let mut constructor = FlipperRef::default();

            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");

            let mut call_builder = contract.call_builder::<Flipper>();

            // Then: 
            let get = call_builder.get_lista_filmes();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            let filmes = get_result.return_value();
            assert_eq!(filmes.is_empty(), true);

            // When: 
            let add_filme = call_builder.delete_filme(String::from("Outro nome de filme!"));

            let err = client.call(&ink_e2e::alice(), &add_filme).submit().await;
            match err{
                Ok(_) =>panic!("Como esse filme não existe, não deveria ser possivel reaaliza-lo"),
                Err(e) =>{
                    assert!(e.to_string().contains("Não existem filmes no sistema!"),
                    "Erro inesperado: {:?}",e);
                },
            };

            // Then: 
            let get_after_add = call_builder.get_lista_filmes();
            let get_result_after_add = client.call(&ink_e2e::alice(), &get_after_add).dry_run().await?;
    
            let filmes_apos_operacao = get_result_after_add.return_value();
            assert_eq!(filmes_apos_operacao.is_empty(), true);

            Ok(())
        }
         #[ink_e2e::test]
       async fn delete_invalid_movie(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given: 
            let mut constructor = FlipperRef::new_with_example();

            let contract = client
                .instantiate("flipper", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");

            let mut call_builder = contract.call_builder::<Flipper>();

            // Then: 
            let get = call_builder.get_lista_filmes();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            let filmes = get_result.return_value();
            assert_eq!(filmes.is_empty(), false);

            // When: 
            let add_filme = call_builder.delete_filme(String::from("Outro nome de filme!?"));

            let err = client.call(&ink_e2e::alice(), &add_filme).submit().await;
            match err{
                Ok(_) =>panic!("Nenhum filme existe com esse nome! Não era para essa operação ocorrer"),
                Err(e) =>{
                    assert!(e.to_string().contains("Não existe um filme com esse nome!\nFilmes disponíveis: "),
                    "Erro inesperado: {:?}",e);
                },
            };

            // Then: 
            let get_after_add = call_builder.get_lista_filmes();
            let get_result_after_add = client.call(&ink_e2e::alice(), &get_after_add).dry_run().await?;
    
            let filmes_apos_operacao = get_result_after_add.return_value();
            assert_eq!(filmes_apos_operacao.len(), filmes.len());

            Ok(())
        }
    }
}
